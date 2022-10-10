use rocket::serde::json::Json;
use types::{FavTeam, FavTeamPayload, Team, NBATeams};

use crate::mongo::{Message, Model};

#[get("/")]
pub(crate) fn index() -> &'static str {
    "Welcome to the NBA teams api"
}

#[get("/teams/insert?<user_id>")]
pub(crate) async fn initialize_nba_teams(user_id: String) {
    Model::generate_nba_teams(user_id).await;
}


#[get("/teams?<user_id>", rank = 1)]
pub(crate) async fn all_nba_teams(user_id: String) -> Json<Vec<NBATeams>> {
    let teams = Model::get_all_teams(user_id).await;

    Json(teams)
}

#[get("/teams/<name>")]
pub(crate) async fn get_team_by_name(name: String) -> Result<Json<Team>, String> {
    let team = Model::get_one_team(name).await;

    Ok(Json(team))
}

// Favorite teams
#[post("/favorite", format = "application/json", data = "<favteam>")]
pub async fn post_favorite_team(favteam: Json<FavTeamPayload>) -> Json<Message> {
    println!("{:?}", favteam);
    let favorite_team = Model::add_favorite_team(favteam).await;

    Json(favorite_team)
}

#[get("/favorite/<user_name>")]
pub async fn get_favorite_teams(user_name: String) -> Json<Vec<FavTeam>> {
    let favorite_teams = Model::get_all_favorite_teams(user_name).await;

    Json(favorite_teams)
}

