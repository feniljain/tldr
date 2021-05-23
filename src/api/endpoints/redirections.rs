use rocket::*;

use crate::pkg::service::TURLService;
use crate::core::Config;

#[get("/<subpath>")]
fn redirect(subpath: String, url_svc: State<TURLService>, config: State<Config>) -> rocket::response::Redirect {
    match url_svc.get_redirection_url(subpath) {
        Ok(redirection_url) => rocket::response::Redirect::temporary(redirection_url),
        Err(_) => rocket::response::Redirect::temporary(config.base_url.clone().to_string()),
    }
}

#[get("/")]
fn health() -> String {
    "Healthy and running!".to_string()
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![health, redirect])
}
