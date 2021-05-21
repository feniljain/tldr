use rocket::*;

use rocket_contrib::json::Json;
use crate::entities;

#[post("/add", format="json", data= "<add_req>")]
fn add(add_req: Json<entities::entities::AddURLReq>) -> String {
    println!("{} : {}", add_req.0.url, add_req.0.redirection_key);
    "Entry added".to_string()
}

#[post("/remove", format="json", data= "<remove_req>")]
fn remove(remove_req: Json<entities::entities::RemoveURLReq>) -> String {
    println!("{}", remove_req.0.url);
    "Entry Removed".to_string()
}

pub fn fuel(rocket: Rocket) -> Rocket {
    rocket.mount("/api", routes![add, remove])
}
