use crate::api;

pub fn rocket() -> rocket::Rocket {
    let mut rocket = rocket::ignite();
    rocket = api::endpoints::urls::fuel(rocket);
    rocket = api::endpoints::utils::fuel(rocket);
    rocket = api::catchers::fuel(rocket);
    rocket
}
