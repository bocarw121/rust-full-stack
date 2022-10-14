use rocket::{http::Status, serde::json::Json};
use types::{FavTeam, FavTeamPayload, NBATeams, Team};

use crate::{
    mongo::{Model},
    responses::{
        error::{Error, ErrorResponse},
        success::{SuccessResponses, Success},
    },
};

#[get("/")]
pub(crate) fn index() -> &'static str {
    "Welcome to the NBA teams api"
}

#[get("/teams/initialize?<user_id>")]
pub(crate) async fn initialize_nba_teams(user_id: String) {
    Model::generate_nba_teams(user_id).await;
}

// TODO:
#[put("/teams?<user_id>&<team_name>")]
pub(crate) async fn update_one_team(user_id: String, team_name: String) -> Result<Success,  Error> {
   let response = match Model::update_one_team(team_name, user_id).await {
    Ok(message) =>   Ok(message),
    Err(error) =>   Err(error)
   };

   response

}

#[get("/teams?<user_id>", rank = 1)]
pub(crate) async fn all_nba_teams(
    user_id: String,
) -> Result<Json<SuccessResponses<Vec<NBATeams>>>, Error> {
    let teams =   match Model::get_all_teams(user_id).await {
        Ok(teams) => {
             Ok(Json(SuccessResponses {
                status: Status::Ok.to_string(),
                data: teams,
            }))
        },
        Err(error) =>  Err(ErrorResponse::create_404_error(&error)),
    };

    teams
}

#[get("/teams/<team_name>?<user_id>")]
pub(crate) async fn get_one_team(
    team_name: String,
    user_id: String,
) -> Result<Json<SuccessResponses<Team>>, Error> {
    let team = match Model::get_one_team(team_name, user_id).await {
        Ok(team) => Ok(Json(SuccessResponses {
            status: Status::Ok.to_string(),
            data: team,
        })),
        // Capture the error from get_one_team
        Err(e) => return Err(ErrorResponse::create_404_error(&e)),
    };

    team
}

// TODO:
// Post  Favorite teams
#[post("/favorite", format = "application/json", data = "<fav_team>")]
pub async fn post_favorite_team(fav_team: Json<FavTeamPayload>) -> Result<Success,  Error>  {
  
    let response =match Model::add_favorite_team(fav_team).await {
        Ok(success) => Ok(success),
        Err(error) =>  Err(error)
    };

    response
}

// TODO:
// Whole favorites team collection
#[get("/favorite?<user_id>")]
pub async fn get_favorite_teams(
    user_id: String,
) -> Result<Json<SuccessResponses<Vec<FavTeam>>>, Error> {
    let favorite_teams = Model::get_all_favorite_teams(user_id).await;

    if favorite_teams.len() == 0 {
        Err(ErrorResponse::create_404_error(
            "No favorite teams selected",
        ))
    } else {
        Ok(Json(SuccessResponses {
            status: Status::Ok.to_string(),
            data: favorite_teams,
        }))
    }
}

// TODO:
#[delete("/favorite?<team_name>&<user_id>")]
pub async fn delete_favorite_team(team_name: String, user_id: String) -> Result<Success,  Error>  {
   let response = match Model::delete_favorite_team(team_name, user_id).await {
    Ok(success) => Ok(success),
    Err(error) => Err(error)
   };

   response
}
