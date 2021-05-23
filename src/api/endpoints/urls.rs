use rocket::{Rocket, State, delete, post, routes};
use rocket_contrib::json::Json;
use anyhow::Result;
use serde_json::{json, Value};

use crate::core;
use crate::pkg::service::TURLService;
use crate::api::guards::CheckAccessToken;

#[post("/add", format="json", data= "<add_req>")]
fn add(add_req: Json<core::AddURLReq>, url_svc: State<TURLService>, _check_access_token: CheckAccessToken) -> Result<Json<Value>> {
    let redirection_url = url_svc.add(add_req.0)?;
    Ok(Json(json!({
        "success":  true,
        "redirectionUrl": redirection_url,
    })))
}

#[delete("/remove", format="json", data= "<remove_req>")]
fn remove(remove_req: Json<core::RemoveURLReq>, url_svc: State<TURLService>, _check_access_token: CheckAccessToken) -> Result<Json<Value>> {
    url_svc.remove(remove_req.0)?;
    Ok(Json(json!({
        "success":  true
    })))
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api", routes![add, remove])
}
