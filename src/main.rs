#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rand;

#[get("/random")]
fn random_number() -> String {
    let random_number = rand::random::<u8>().to_string();
    format!("{{ 'random_n umber': {}' }}", random_number)
}

fn main() {
    rocket::ignite().mount("/", routes![random_number]).launch();
}
