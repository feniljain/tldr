#![feature(proc_macro_hygiene, decl_macro)]

mod entities;
mod rocket_builder;
mod api;
mod pkg;

fn main() {
    rocket_builder::rocket().launch();
}
