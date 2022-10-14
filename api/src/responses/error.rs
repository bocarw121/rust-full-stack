use rocket::{http::Status, serde::json::Json};
use serde::Serialize;
#[derive(Debug, Serialize, Responder)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
   pub fn create_400_error(message: &str) -> Error {
        Error::BadRequest(Json(ErrorResponse {
            message: format!(
                "Status code: {} message: {}",
                Status::BadRequest.to_string(),
                message
            ),
        }))
    }
    pub fn create_404_error(message: &str) -> Error {
        Error::NotFoundError(Json(ErrorResponse {
            message: format!(
                "Status code: {} message: {}",
                Status::NotFound.to_string(),
                message
            ),
        }))
    }

    pub fn create_409_error(message: &str) -> Error {
        Error::DuplicateError(Json(ErrorResponse {
            message: format!(
                "Status code: {} message: {}",
                Status::Conflict.to_string(),
                message
            ),
        }))
    }

    pub fn create_500_error(message: &str) -> Error {
        Error::InternalError(Json(ErrorResponse {
            message: format!(
                "Status code: {} message: {}",
                Status::InternalServerError.to_string(),
                message
            ),
        }))
    }
}

#[derive(Debug, Responder)]
pub enum Error {
  #[response(status = 400, content_type = "json")]
    BadRequest(Json<ErrorResponse>),
    #[response(status = 404, content_type = "json")]
    NotFoundError(Json<ErrorResponse>),
    #[response(status = 500, content_type = "json")]
    InternalError(Json<ErrorResponse>),
    #[response(status = 409, content_type = "json")]
    DuplicateError(Json<ErrorResponse>),
}

// Message {
//                 status: Status::NotFound.to_string(),
//                 message: "Team not found".to_owned()
//             }
