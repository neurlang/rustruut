use crate::interfaces::{PolicyMaxWords, IpaFlavor, DictGetter, Api};
use crate::di::DependencyInjection;
use std::collections::HashMap;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::{requests, responses};

use super::config::{Config};
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

pub struct Goruut<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    policy: P,
    ipa: I,
    dict_getter: D,
    executable: Option<Executable>,
    platform: Option<Platform>,
    version: Option<String>,
    process: Option<Child>,
    config: Config<P, I, D, A>,
}

impl<P, I, D, A> Goruut<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    pub fn new(di: DependencyInjection<P, I, D, A>, version: Option<&str>, writeable_bin_dir: Option<&str>, api: Option<&str>, models: HashMap<String, String>) -> Result<Self, RustruutError> {
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

        let executable = executable.ok_or_else(|| RustruutError::Platform("No executable found for platform".to_string()))?;
        let version = version_found.ok_or_else(|| RustruutError::Platform("Version not found".to_string()))?;

        let temp_dir = if let Some(dir) = writeable_bin_dir {
            TempDir::new_in(dir)?
        } else {
            let home = dirs::home_dir().ok_or_else(|| RustruutError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")))?;
            let goruut_dir = home.join(".goruut");
            std::fs::create_dir_all(&goruut_dir)?;
            TempDir::new_in(goruut_dir)?
        };

        let executable_path = match executable.exists(temp_dir.path()) {
            Ok(path) => path,
            Err(_) => executable.download(temp_dir.path())?,
        };

        let config = Config::new(di.clone());
        let config_path = temp_dir.path().join("goruut_config.json");
        config.serialize(config_path.to_str().unwrap(), &models)?;

        let process = Command::new(executable_path)
            .arg("--configfile")
            .arg(config_path.to_str().unwrap())
            .spawn()?;

        // Wait for the process to start serving
        thread::sleep(Duration::from_secs(2));

        Ok(Self {
            policy: di.policy.clone(),
            ipa: di.ipa.clone(),
            dict_getter: di.dict_getter.clone(),
            executable: Some(executable),
            platform: Some(platform),
            version: Some(version),
            process: Some(process),
            config: config,
        })
    }

    pub fn phonemize(&self, req: requests::PhonemizeSentence) -> Result<responses::PhonemizeSentence, RustruutError> {
        let url = self.config.url("tts/phonemize/sentence");
        
        let payload = serde_json::json!(req);

        let client = reqwest::blocking::Client::new();
        let response = client.post(&url).json(&payload).send()?;

        let data: serde_json::Value = response.json()?;

        let resp: responses::PhonemizeSentence = serde_json::from_value(data.clone())?;

        Ok(resp)
    }
}

impl<P, I, D, A> Drop for Goruut<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    fn drop(&mut self) {
        if let Some(process) = &mut self.process {
            let _ = process.kill();
        }
    }
}
