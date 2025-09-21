use std::collections::HashMap;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;
use tempfile::TempDir;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::{requests, responses};

use super::config::{Config, ConfigApi};
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

pub struct Goruut {
    executable: Option<Executable>,
    platform: Option<Platform>,
    version: Option<String>,
    process: Option<Child>,
    config: Box<dyn GoruutConfig>,
}

trait GoruutConfig {
    fn url(&self, subpath: &str) -> String;
}

impl GoruutConfig for Config {
    fn url(&self, subpath: &str) -> String {
        Config::url(self, subpath)
    }
}

impl GoruutConfig for ConfigApi {
    fn url(&self, subpath: &str) -> String {
        ConfigApi::url(self, subpath)
    }
}

impl Goruut {
    pub fn new(version: Option<&str>, writeable_bin_dir: Option<&str>, api: Option<&str>, models: HashMap<String, String>) -> Result<Self, RustruutError> {
        if let Some(api_url) = api {
            let config = Box::new(ConfigApi::new(api_url));
            return Ok(Self {
                executable: None,
                platform: None,
                version: None,
                process: None,
                config,
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

        let config = Config::new();
        let config_path = temp_dir.path().join("goruut_config.json");
        config.serialize(config_path.to_str().unwrap(), &models)?;

        let process = Command::new(executable_path)
            .arg("--configfile")
            .arg(config_path.to_str().unwrap())
            .spawn()?;

        // Wait for the process to start serving
        thread::sleep(Duration::from_secs(2));

        let config_box: Box<dyn GoruutConfig> = Box::new(config);

        Ok(Self {
            executable: Some(executable),
            platform: Some(platform),
            version: Some(version),
            process: Some(process),
            config: config_box,
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

impl Drop for Goruut {
    fn drop(&mut self) {
        if let Some(process) = &mut self.process {
            let _ = process.kill();
        }
    }
}
