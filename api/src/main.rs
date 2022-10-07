#[macro_use] extern crate rocket;

pub mod routes;
pub mod nba;
use routes::*;




#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, all_nba_teams])
}
