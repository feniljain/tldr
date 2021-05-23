use rocket::{State, request::FromRequest};
use rocket::outcome::Outcome::*;

use crate::core::Config;

#[derive(Debug)]
pub struct CheckAccessToken(Config);

impl<'a, 'r> FromRequest<'a, 'r> for CheckAccessToken {
    type Error = ();

    fn from_request(request: &'a rocket::Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
        let mut value = request.headers().get("Authorization");
        if let Some(key) = value.next() {
            let config_guard = request.guard::<State<Config>>();
            if config_guard.is_success() {
                if let Some(config) = config_guard.succeeded() {
                    if config.secret_key.as_str() != key {
                        return Failure((rocket::http::Status::Unauthorized, ()));
                    }
                    return Success(CheckAccessToken(config.clone()));
                }
            }
            return Failure((rocket::http::Status::InternalServerError, ()));
        }

        Failure((rocket::http::Status::Unauthorized, ()))
    }
}
