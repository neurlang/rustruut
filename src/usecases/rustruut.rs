use crate::di::DependencyInjection;
use crate::interfaces::{Api, DictGetter, Folder, IpaFlavor, PolicyMaxWords};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;
use thiserror::Error;

use crate::models::{requests, responses};

use super::config::Config;
use super::executable::{Executable, ExecutableError};
use super::platform::Platform;
use super::release::get_releases;

#[derive(Error, Debug)]
pub enum RustruutError {
    #[error("Executable error: {0}")]
    Executable(#[from] ExecutableError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Platform error: {0}")]
    Platform(String),
    #[error("Process error: {0}")]
    Process(String),
    #[error("Generic error: {0}")]
    Generic(String),
    // Add the new variant here
    #[error("Serde JSON error: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Word {
    pub clean_word: String,
    pub phonetic: String,
    pub pos_tags: Vec<String>,
    pub pre_punct: String,
    pub post_punct: String,
    pub is_first: bool,
    pub is_last: bool,
}

#[derive(Debug)]
pub struct PhonemeResponse {
    pub words: Vec<Word>,
    pub separator: String,
}

impl ToString for PhonemeResponse {
    fn to_string(&self) -> String {
        self.words
            .iter()
            .map(|w| format!("{}{}{}", w.pre_punct, w.phonetic, w.post_punct))
            .collect::<Vec<String>>()
            .join(&self.separator)
    }
}

// A global mutex used to protect downloading (critical section)
static DOWNLOAD_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

pub struct Goruut<P, I, D, A, F>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
{
    policy: P,
    ipa: I,
    dict_getter: D,
    executable: Option<Executable>,
    platform: Option<Platform>,
    version: Option<String>,
    process: Option<Child>,
    config: Config<P, I, D, A, F>,
}

impl<P, I, D, A, F> Goruut<P, I, D, A, F>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
{
    fn download_critical(executable: &Executable, p: &Path) -> Result<PathBuf, RustruutError> {
        let _guard = DOWNLOAD_LOCK.lock().unwrap();
        let executable_path = match executable.exists(p) {
            Ok(path) => path,
            Err(_) => executable.download(p)?,
        };
        return Ok(executable_path);
    }

    pub fn new(
        di: DependencyInjection<P, I, D, A, F>,
        version: Option<&str>,
        writeable_bin_dir: Option<&str>,
        api: Option<&str>,
        models: HashMap<String, String>,
    ) -> Result<Self, RustruutError> {
        if di.api.get_api_path().len() != 0 {
            let config = Config::new(di.clone());
            return Ok(Self {
                policy: di.policy.clone(),
                ipa: di.ipa.clone(),
                dict_getter: di.dict_getter.clone(),
                executable: None,
                platform: None,
                version: None,
                process: None,
                config: config,
            });
        }

        let platform = Platform::new().map_err(|e| RustruutError::Platform(e.to_string()))?;
        let releases = get_releases();
        let mut executable = None;
        let mut version_found = None;

        for release in releases {
            if let Some(ver) = version {
                if !release.version.starts_with(ver) {
                    continue;
                }
            }

            if release.architecture == platform.architecture && release.os == platform.os {
                executable = Some(Executable {
                    size: release.size,
                    sha256: release.sha256,
                    architecture: release.architecture,
                    os: release.os,
                    servers: release.servers,
                });
                version_found = Some(release.version);
                break;
            }
        }

        let executable = executable.ok_or_else(|| {
            RustruutError::Platform("No executable found for platform".to_string())
        })?;
        let version = version_found
            .ok_or_else(|| RustruutError::Platform("Version not found".to_string()))?;

        let temp_dir = if writeable_bin_dir == None {
            std::env::temp_dir()
        } else if writeable_bin_dir.as_deref().map_or(true, |s| s.is_empty()) {
            let home = dirs::home_dir().ok_or_else(|| {
                RustruutError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found",
                ))
            })?;
            let goruut_dir = home.join(".goruut");
            std::fs::create_dir_all(&goruut_dir)?;
            goruut_dir
        } else {
            PathBuf::from(writeable_bin_dir.unwrap_or(""))
        };

        let executable_path = Self::download_critical(&executable, &temp_dir)?;

        let config = Config::new(di.clone());
        let config_path = temp_dir.join(format!("goruut_config_{}.json", config.get_port()));
        config.serialize(config_path.to_str().unwrap(), &models)?;

        // Inside your function
        let mut child = Command::new(executable_path)
            .arg("--configfile")
            .arg(config_path.to_str().unwrap())
            .stderr(Stdio::piped()) // Capture stderr
            .spawn()?;

        // Take ownership of the child's stderr
        let stderr = child.stderr.take().expect("Failed to capture stderr");

        // Create a channel for thread communication
        let (tx, rx) = channel();

        // Spawn a thread to read stderr
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                match line {
                    Ok(line) => {
                        if line.contains("Serving...") {
                            tx.send(Some(())).expect("Failed to send message");
                            return;
                        }
                    }
                    Err(_) => break, // Stop on read error
                }
            }
            // Send None if EOF reached without finding the message
            tx.send(None).expect("Failed to send message");
        });

        // Wait for either the message or process exit
        let result = match rx.recv_timeout(Duration::from_secs(30)) {
            Ok(Some(())) => {
                // Found "Serving..." message
                let s = Self {
                    policy: di.policy.clone(),
                    ipa: di.ipa.clone(),
                    dict_getter: di.dict_getter.clone(),
                    executable: Some(executable),
                    platform: Some(platform),
                    version: Some(version),
                    process: Some(child),
                    config: config,
                };
                /*
                        // Send one dummy request and end
                        let url = s.config.url("tts/phonemize/sentence");
                        let payload = "{}".to_string();

                        let client = reqwest::blocking::Client::new();
                        let _ = client.post(&url).json(&payload).send();

                    // Usage - parse from empty JSON
                    let mut req: requests::PhonemizeSentence = serde_json::from_str("{}")?;

                        req.init();

                        let _ = s.phonemize(req);
                */
                return Ok(s);
            }
            Ok(None) => {
                // Stderr closed without finding the message
                return Err(RustruutError::Process(
                    "Process exited without serving message".into(),
                ));
            }
            Err(RecvTimeoutError::Timeout) => {
                // Timeout reached
                return Err(RustruutError::Process(
                    "Timeout waiting for serving message".into(),
                ));
            }
            Err(RecvTimeoutError::Disconnected) => {
                // Thread panicked or closed channel
                return Err(RustruutError::Process(
                    "Stderr reader thread disconnected".into(),
                ));
            }
        };

        // Check if process is still running
        if let Ok(Some(status)) = child.try_wait() {
            return Err(RustruutError::Process(format!(
                "Process exited early with status: {}",
                status
            )));
        }

        Ok(Self {
            policy: di.policy.clone(),
            ipa: di.ipa.clone(),
            dict_getter: di.dict_getter.clone(),
            executable: Some(executable),
            platform: Some(platform),
            version: Some(version),
            process: Some(child),
            config: config,
        })
    }

    pub fn phonemize(
        &self,
        req: requests::PhonemizeSentence,
    ) -> Result<responses::PhonemizeSentence, RustruutError> {
        let url = self.config.url("tts/phonemize/sentence");

        let payload = serde_json::json!(req);

        let client = reqwest::blocking::Client::new();
        let response = client.post(&url).json(&payload).send()?;

        let data: serde_json::Value = response.json()?;

        let resp: responses::PhonemizeSentence = serde_json::from_value(data.clone())?;

        Ok(resp)
    }
}

impl<P, I, D, A, F> Drop for Goruut<P, I, D, A, F>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
    F: Folder,
{
    fn drop(&mut self) {
        if let Some(process) = &mut self.process {
            let _ = process.kill();
        }
    }
}
