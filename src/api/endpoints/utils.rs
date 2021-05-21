// #![feature(decl_macro)]

use rocket::*;

#[get("/")]
fn health() -> String {
    "Healthy and running!".to_string()
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![health])
}
