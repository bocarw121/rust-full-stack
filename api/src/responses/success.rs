use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
use types::{NBATeams, Team};

#[derive(Debug, Serialize, Responder)]
pub struct SuccessResponses<T> {
    pub status: String,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponsesNoData {
    pub status: String,
   pub message: String,
}

pub struct APIResponses;

impl SuccessResponsesNoData {
    pub fn ok(message: String) -> Success {
        Success::Ok(Json(SuccessResponsesNoData {
            status: Status::Ok.to_string(),
            message
        }))
    }
    pub fn created(message: String) -> Success {
        Success::Created(Json(SuccessResponsesNoData {
            status: Status::Created.to_string(),
            message
        }))
    }
    pub fn updated(message: String) -> Success {
        Success::Updated(Json(SuccessResponsesNoData {
            status: Status::Ok.to_string(),
            message
        }))
    }
}

#[derive(Debug, Responder)]
pub enum Success {
    #[response(status = 200, content_type = "json")]
    Ok(Json<SuccessResponsesNoData>),
    #[response(status = 200, content_type = "json")]
    Created(Json<SuccessResponsesNoData>),
    #[response(status = 201, content_type = "json")]
    Updated(Json<SuccessResponsesNoData>),
}
