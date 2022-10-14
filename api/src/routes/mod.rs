use rocket::{serde::json::Json, http::Status};
use types::{FavTeam, FavTeamPayload, NBATeams, Team};

use crate::{mongo::{Message, Model}, responses::{success::SuccessResponses, error::{Error, ErrorResponse}}};

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
pub(crate) async fn all_nba_teams(user_id: String) -> Result<Json<SuccessResponses<Vec<NBATeams>>>, Error> {
    let teams = Model::get_all_teams(user_id).await;

   if teams.len() == 0 {
     Err(ErrorResponse::create_404_error("Teams, not found"))
   } else {
    Ok(Json(SuccessResponses {
      status: Status::Ok.to_string(),
      data: teams
    }))
   }
}

#[get("/teams/<team_name>?<user_id>")]
pub(crate) async fn get_one_team(team_name:String, user_id: String) -> Result<Json<SuccessResponses<Team>>, Error> {
    let team = match Model::get_one_team(team_name, user_id).await {
        Ok(team) => Ok(Json(SuccessResponses {
      status: Status::Ok.to_string(),
      data: team
    })),
        // Capture the error from get_one_team
        Err(e) => return Err(ErrorResponse::create_404_error(&e))
    };

    team

}

// Post  Favorite teams
#[post("/favorite", format = "application/json", data = "<fav_team>")]
pub async fn post_favorite_team(fav_team: Json<FavTeamPayload>) -> Json<Message> {
    println!("{:?}", fav_team);
    let favorite_team = Model::add_favorite_team(fav_team).await;

    Json(favorite_team)
}

// Whole favorites team collection
#[get("/favorite?<user_id>")]
pub async fn get_favorite_teams(user_id: String) -> Result<Json<SuccessResponses<Vec<FavTeam>>>, Error> {
    let favorite_teams  = Model::get_all_favorite_teams(user_id).await;

    if favorite_teams.len() == 0 {
        Err(ErrorResponse::create_404_error("No favorite teams selected"))
    } else {
         Ok(Json(SuccessResponses { status: Status::Ok.to_string(), data: favorite_teams }))
    }

 
   
}


#[delete("/favorite?<team_name>&<user_id>")]
pub async fn delete_favorite_team(team_name:String, user_id: String) -> Status{
  Model::delete_favorite_team(team_name, user_id).await
}


