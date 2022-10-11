use rocket::serde::json::Json;
use types::{FavTeam, FavTeamPayload, NBATeams, Team};

use crate::mongo::{Message, Model};

#[get("/")]
pub(crate) fn index() -> &'static str {
    "Welcome to the NBA teams api"
}

#[get("/teams/insert?<user_id>")]
pub(crate) async fn initialize_nba_teams(user_id: String) {
    Model::generate_nba_teams(user_id).await;
}

#[put("/teams?<user_id>&<team_name>")]
pub(crate) async fn update_one_team(user_id: String,team_name:String) -> Json<Message> {
    Json(Model::update_one_team(team_name, user_id).await)
}

#[get("/teams?<user_id>", rank = 1)]
pub(crate) async fn all_nba_teams(user_id: String) -> Json<NBATeams> {
    let teams = Model::get_all_teams(user_id).await;

    Json(NBATeams {
        _id: teams[0]._id.clone(),
        teams: teams[0].teams.clone(),
    })
}

#[get("/teams/<name>?<user_id>")]
pub(crate) async fn get_team_by_name(name: String, user_id: String) -> Result<Json<Team>, String> {
    let team = match Model::get_one_team(name, user_id).await {
        Ok(team) => team,
        Err(_) => return Err("Team not found".to_owned())
    };

    Ok(Json(team))
}

// Post  Favorite teams
#[post("/favorite", format = "application/json", data = "<favteam>")]
pub async fn post_favorite_team(favteam: Json<FavTeamPayload>) -> Json<Message> {
    println!("{:?}", favteam);
    let favorite_team = Model::add_favorite_team(favteam).await;

    Json(favorite_team)
}

// Whole favorites team collection
#[get("/favorite?<user_id>")]
pub async fn get_favorite_teams(user_id: String) -> Json<Vec<FavTeam>> {
    let favorite_teams = Model::get_all_favorite_teams(user_id).await;

    Json(favorite_teams)
}

