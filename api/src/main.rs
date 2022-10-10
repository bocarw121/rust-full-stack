#[macro_use]
extern crate rocket;

pub mod mongo;
pub mod nba;
pub mod routes;
pub mod utils;
use routes::*;

#[launch]
fn rocket() -> _ {
    // Mount paths for index and nba routes
    rocket::build().mount("/", routes![index]).mount(
        "/nba",
        routes![
            all_nba_teams,
            get_team_by_name,
            post_favorite_team,
            get_favorite_teams
        ],
    )
}
