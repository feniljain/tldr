use anyhow::{anyhow, Result}; use regex::RegexBuilder;
use serde_json::{Map, Value};
use std::fs;

use crate::core::{AddURLReq, Config, RemoveURLReq};
use crate::pkg::repository::{IURLRepository, TURLRepository};

pub struct DBJson {
    pub config: Config,
}

impl DBJson {
    pub fn new(config: Config) -> TURLRepository {
        Box::new(DBJson { config })
    }
}

impl IURLRepository for DBJson {
    fn add(&self, req: AddURLReq) -> Result<String> {
        if let Ok(json) = read_json(self.config.db_key.clone()) {
            let mut json = json.to_owned();

            let re_url = RegexBuilder::new(
                r"https?://(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()@:%_\+.~#?&//=]*)",
            ).build();
            match re_url {
                Ok(re) => {
                    if re.is_match(&req.url) {
                        json.insert(
                            req.redirection_key.as_str().to_string(),
                            Value::String(req.url),
                        );
                        let json_str = serde_json::to_string(&json)?;
                        fs::write(self.config.db_key.clone(), json_str)?;
                        return Ok("redirection added!".to_string());
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    return Err(anyhow!("error: invalid url format"));
                }
            }
        }

        Err(anyhow!("error deserializing json"))
    }

    fn remove(&self, req: RemoveURLReq) -> Result<()> {
        if let Ok(json) = read_json(self.config.db_key.clone()) {
            let mut json = json.to_owned();
            json.remove(&req.redirection_key);
            let json_str = serde_json::to_string(&json)?;
            fs::write(self.config.db_key.clone(), json_str)?;
            return Ok(());
        }

        Err(anyhow!("error: error reading json file"))
    }

    fn get_redirection_url(&self, key: String) -> Result<String> {
        if let Ok(json) = read_json(self.config.db_key.clone()) {
            if let Some(redirection_value) = json.get(&key) {
                if redirection_value.is_string() {
                    let redirection_url = redirection_value.as_str().unwrap();
                    return Ok(redirection_url.to_string());
                }
            } else {
                return Err(anyhow!("error: key not found"));
            }
        }

        Err(anyhow!("error: internal server error"))
    }
}

fn read_json(path: String) -> Result<Map<String, Value>> {
    let json_str = fs::read_to_string(path)?;
    let data: Value = serde_json::from_str(&json_str)?;

    if !data.is_object() {
        return Err(anyhow!("invalid json"));
    }

    if let Some(json) = data.as_object() {
        return Ok(json.to_owned());
    }

    return Err(anyhow!("error reading json file"));
}
