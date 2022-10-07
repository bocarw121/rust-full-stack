#[macro_use] extern crate rocket;

pub mod routes;
use routes::*;


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
