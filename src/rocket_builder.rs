use anyhow::{ Result, anyhow };

use std::env;
use crate::api;
use crate::core::Config;
use crate::pkg::service::URLService;

pub fn rocket() -> Result<rocket::Rocket> {
    let mut base_url = String::new();
    let mut secret_key = String::new();
    for (key, value) in env::vars() {
        if key == "BASE_URL" {
            base_url = value.clone();
        }
        if key == "SECRET_KEY" {
            secret_key = value.clone();
        }
    }

    let config = Config {
        db_type: "json".to_string(),
        db_key: "./urls.json".to_string(),
        base_url,
        secret_key,
    };

    if let Ok(url_service) = URLService::new(config.clone()) {

        let mut rocket = rocket::ignite();
        rocket = api::endpoints::urls::fuel(rocket);
        rocket = api::endpoints::redirections::fuel(rocket);
        rocket = api::catchers::fuel(rocket);
        rocket = rocket.manage(url_service);
        rocket = rocket.manage(config.clone());


        return Ok( rocket );
    }

    Err(anyhow!("error initializing url service"))
}
