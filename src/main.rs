#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rand;

mod weberror;
use weberror::web_error::Error;

fn generate_a_random_number(min: f32,max:f32)-> Result<i32,Error>{
    if max <min{
        return Err(Error::new("InternalServerError", "max has to be greater than min"));
    }
    let random_number = rand::random::<f32>()*(max-min)+min;
    Ok(random_number as i32)
}

#[get("/random")]
fn random_number() -> String {
    let random_number = match generate_a_random_number(22.0, 20.0){
        Ok(num) => num,
        Err(err) => return format!("{}",err)
    };
    format!("{{ 'random number': {}' }}", random_number)
}

fn main() {
    rocket::ignite().mount("/", routes![random_number]).launch();
}
