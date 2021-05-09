#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::{get, route};
#[macro_use]
extern crate magic;
use rand;

#[get("/random")]
fn random_number() -> String {
    let random_number = rand::random::<u8>().to_string();
    format!("{{ 'random_n umber': {}' }}", random_number)
}

#[get_("/ok")]
fn try_this() -> String {
    "wOw".to_string()
}

fn main() {
    try_this();
    rocket::ignite().mount("/", routes![random_number]).launch();
}
