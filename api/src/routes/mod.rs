use rocket::{http::Status, serde::json::Json};
use types::{NBATeams, Team};

use crate::nba;

#[get("/")]
pub(crate) fn index() -> &'static str {
    "Welcome to the NBA teams api"
}

#[get("/teams")]
pub(crate) fn all_nba_teams() -> Json<NBATeams> {
    Json(NBATeams {
        teams: nba::get_teams(),
    })
}

#[get("/teams/<name>")]
pub(crate) fn get_team_by_name(name: String) -> Result<Json<Team>, Status> {
    let nba_teams = NBATeams {
        teams: nba::get_teams(),
    }
    .teams;
    // Loop through nba_teams vec to locate team Team matching the name given
    // and returns a 404 if no team is found
    let team = match nba_teams.iter().find(|team| team.name.to_lowercase() == name) {
        Some(team) => team,
        None => return Err(Status::NotFound),
    };
    // Return a copy of the team as JSON
    Ok(Json(team.clone()))
}
