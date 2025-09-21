use crate::di::DependencyInjection;
use crate::interfaces::{Api, DictGetter, IpaFlavor, PolicyMaxWords};
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
    policy_max_words: usize,
    load_models: Option<Vec<LoadModel>>,
}

pub struct Config<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    policy: P,
    ipa: I,
    dict: D,
    api: A,
    port: u16,
}

impl<P, I, D, A> Config<P, I, D, A>
where
    P: PolicyMaxWords,
    I: IpaFlavor,
    D: DictGetter,
    A: Api,
{
    pub fn new(di: DependencyInjection<P, I, D, A>) -> Self {
        let port = rand::thread_rng().gen_range(1024..=65535);
        Self {
            policy: di.policy.clone(),
            ipa: di.ipa.clone(),
            dict: di.dict_getter.clone(),
            api: di.api.clone(),
            port: port,
        }
    }

    pub fn serialize(
        &self,
        filename: &str,
        models: &HashMap<String, String>,
    ) -> std::io::Result<()> {
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
            policy_max_words: self.policy.get_policy_max_words(),
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
        if self.api.get_api_path().len() == 0 {
            format!("http://127.0.0.1:{}/{}", self.port, subpath)
        } else {
            format!("{}{}/{}", self.api.get_api_path(), self.port, subpath)
        }
    }
}
