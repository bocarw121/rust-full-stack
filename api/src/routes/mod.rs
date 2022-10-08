use rocket::serde::json::Json;
use types::{FavTeam, NBATeams, Team};

use crate::mongo::{ErrorMessage, Model};

#[get("/")]
pub(crate) fn index() -> &'static str {
    "Welcome to the NBA teams api"
}

#[get("/teams")]
pub(crate) async fn all_nba_teams() -> Json<NBATeams> {
    let teams = Model::get_all_teams().await;

    Json(NBATeams { teams })
}

#[get("/teams/<name>")]
pub(crate) async fn get_team_by_name(name: String) -> Result<Json<Team>, String> {
    let team = Model::get_one_team(name).await;

    Ok(Json(team))
}

// Favorite teams
#[post("/favorite", format = "application/json", data = "<favteam>")]
pub async fn post_favorite_team(favteam: Json<FavTeam>) -> Json<ErrorMessage> {
    Json(Model::add_favorite_team(favteam).await)
}
