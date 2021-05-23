#![feature(proc_macro_hygiene, decl_macro)]

mod core;
mod rocket_builder;
mod api;
mod pkg;

fn main() {
    match rocket_builder::rocket() {
        Ok(rocket) => {
            rocket.launch();
        },
        Err(err) => {
            println!("{}", err);
            panic!();
        },
    }
}
