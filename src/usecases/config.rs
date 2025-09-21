use rand::Rng;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct LoadModel {
    lang: String,
    file: String,
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct ConfigData {
    port: String,
    admin_port: String,
    policy_max_words: u32,
    load_models: Option<Vec<LoadModel>>,
}

pub struct Config {
    port: u16,
}

impl Config {
    pub fn new() -> Self {
        let port = rand::thread_rng().gen_range(1024..=65535);
        Self { port }
    }

    pub fn serialize(&self, filename: &str, models: &HashMap<String, String>) -> std::io::Result<()> {
        let mut load_models = Vec::new();
        for (lang, file) in models {
            load_models.push(LoadModel {
                lang: lang.clone(),
                file: file.clone(),
            });
        }

        let data = ConfigData {
            port: self.port.to_string(),
            admin_port: (self.port - 1).to_string(),
            policy_max_words: 9999999,
            load_models: if load_models.is_empty() {
                None
            } else {
                Some(load_models)
            },
        };

        let mut file = File::create(filename)?;
        let json = serde_json::to_string_pretty(&data)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn url(&self, subpath: &str) -> String {
        format!("http://127.0.0.1:{}/{}", self.port, subpath)
    }
}

pub struct ConfigApi {
    base_url: String,
}

impl ConfigApi {
    pub fn new(url: &str) -> Self {
        Self {
            base_url: url.to_string(),
        }
    }

    pub fn url(&self, subpath: &str) -> String {
        format!("{}/{}", self.base_url, subpath)
    }
}
