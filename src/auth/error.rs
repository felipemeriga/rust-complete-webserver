use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use actix_web_httpauth::headers::www_authenticate::bearer::Bearer;
use derive_more::Display;

use crate::models::error::ErrorMessage;
use awc::error::{JsonPayloadError, SendRequestError};
use jsonwebtoken::jwk::AlgorithmParameters;

#[derive(Debug, Display)]
pub enum ClientError {
    #[display(fmt = "authentication")]
    Authentication(actix_web_httpauth::extractors::AuthenticationError<Bearer>),
    #[display(fmt = "send_request_error")]
    SendRequestError(SendRequestError),
    #[display(fmt = "decode")]
    Decode(jsonwebtoken::errors::Error),
    #[display(fmt = "not_found")]
    NotFound(String),
    #[display(fmt = "unsupported_algorithm")]
    UnsupportedAlgortithm(AlgorithmParameters),
    #[display(fmt = "json_payload_error")]
    JsonPayloadError(JsonPayloadError),
    #[display(fmt = "no_permission")]
    NoPermission(String),
}

impl ResponseError for ClientError {
    fn status_code(&self) -> StatusCode {
        StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Self::JsonPayloadError(err) => HttpResponse::InternalServerError().json(ErrorMessage {
                error: Option::from(err.to_string()),
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::SendRequestError(err) => HttpResponse::InternalServerError().json(ErrorMessage {
                error: Option::from(err.to_string()),
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::Authentication(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: None,
                error_description: None,
                message: "Requires authentication".to_string(),
            }),
            Self::Decode(_) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(
                    "Authorization header value must follow this format: Bearer access-token"
                        .to_string(),
                ),
                message: "Bad credentials".to_string(),
            }),
            Self::NoPermission(access_level) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("lack of permissions".to_string()),
                error_description: Some(format!(
                    "Your user doesn't have {} permission",
                    access_level
                )),
                message: "Bad credentials".to_string(),
            }),
            Self::NotFound(msg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(msg.to_string()),
                message: "Bad credentials".to_string(),
            }),
            Self::UnsupportedAlgortithm(alg) => HttpResponse::Unauthorized().json(ErrorMessage {
                error: Some("invalid_token".to_string()),
                error_description: Some(format!(
                    "Unsupported encryption algortithm expected RSA got {:?}",
                    alg
                )),
                message: "Bad credentials".to_string(),
            }),
        }
    }
}
