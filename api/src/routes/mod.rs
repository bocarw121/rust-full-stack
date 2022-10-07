use rocket::serde::json::Json;
use types::NBATeams;

use crate::nba;



#[get("/")]
pub(crate) fn index() -> &'static str {
    "Welcome to the NBA teams api"
}

#[get("/nba/teams")]
pub(crate) fn all_nba_teams() -> Json<NBATeams> {
    Json(NBATeams { teams: nba::get_teams() })
}
