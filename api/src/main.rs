#[macro_use]
extern crate rocket;

pub mod mongo;
pub mod nba;
pub mod responses;
pub mod routes;
pub mod utils;
use routes::*;

#[launch]
fn rocket() -> _ {
    // Mount paths for index and nba routes
    rocket::build().mount("/", routes![index]).mount(
        "/nba",
        routes![
            initialize_nba_teams,
            all_nba_teams,
            get_one_team,
            post_favorite_team,
            get_favorite_teams,
            update_one_team,
            delete_favorite_team
        ],
    )
}
