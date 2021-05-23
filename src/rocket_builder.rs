use anyhow::{ Result, anyhow };

use crate::api;
use crate::core::Config;
use crate::pkg::service::URLService;

pub fn rocket() -> Result<rocket::Rocket> {

    let config = Config {
        db_type: "json".to_string(),
        db_key: "./urls.json".to_string(),
        base_url: "https://ieeevit.org".to_string(),
        secret_key: "IEEE-VIT".to_string(),
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
