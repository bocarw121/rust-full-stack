#[macro_use]
extern crate rocket;

pub mod nba;
pub mod routes;
use routes::*;

#[launch]
fn rocket() -> _ {
    // Mount paths for index and nba routes
    rocket::build().mount("/", routes![index]).mount("/nba", routes![all_nba_teams,get_team_by_name])
}
