use rocket::{Rocket, catch, catchers, http::Status, response::status};
use rocket_contrib::json::Json;
use serde_json::{json, Value};

#[catch(400)]
fn bad_request() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::BadRequest,
        Json(json!({
            "err": "bad request",
            "msg": "Error parsing JSON body",
        })),
    )
}

#[catch(404)]
fn not_found() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::NotFound,
        Json(json!({
            "err":"route not found",
            "msg": "The given route does not exist"
        })),
    )
}

#[catch(422)]
fn unprocessable_entity() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::UnprocessableEntity,
        Json(json!({
            "err":"failed to process entity",
            "msg": "The given object could not be processed. This could be due to sending \
             malformed or incomplete JSON objects in the request body."
        })),
    )
}

#[catch(500)]
fn internal_server_error() -> status::Custom<Json<Value>> {
    status::Custom(
        Status::NotFound,
        Json(json!({
            "err":"internal server error",
            "msg": "Something went wrong, try again"
        })),
    )
}


pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.register(
        catchers![
            bad_request,
            not_found,
            unprocessable_entity,
            internal_server_error,
        ])
}
