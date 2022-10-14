use rocket::{serde::json::Json, http::Status,  };
use serde::Serialize;
use types::{Team, NBATeams};




#[derive(Debug,Serialize, Responder)]
pub struct SuccessResponses<T> {
 pub status: String,
 pub data: T
}

pub struct APIResponses;

impl APIResponses {
  pub fn get_teams(teams: Vec<NBATeams>) -> Success {
    Success::GetTeams(Json(SuccessResponses {
      status: Status::Ok.to_string(),
      data: teams
    }))
}
}

#[derive(Debug, Responder)]
pub enum Success {
  #[response(status = 200)]
  GetTeams(Json<SuccessResponses<Vec<NBATeams>>>),
  // #[response(status = 200)]
  // PostTeam(Json<SuccessResponses<>>),
  // #[response(status = 201, content_type = "json")]
  // Created(Json<SuccessResponses>),
}

