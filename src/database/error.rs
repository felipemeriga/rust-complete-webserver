use crate::models::error::ErrorMessage;
use actix_web::{HttpResponse, ResponseError};
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum RepositoryError {
    // Using Trait std::error::Error, because as we are abstracting our database, different implementations of the database trait, may return different errors
    CreateUpdateUser(Box<dyn Error>),
    DeleteUser(Box<dyn Error>),
    GeneralError(String),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::CreateUpdateUser(err) => write!(f, "Error saving data to Database: {}", err),
            Self::GeneralError(msg) => write!(f, "{}", msg),
            Self::DeleteUser(err) => write!(f, "Error deleting data in the Database: {}", err),
        }
    }
}

impl ResponseError for RepositoryError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::CreateUpdateUser(err) => HttpResponse::InternalServerError().json(ErrorMessage {
                error: Option::from(err.to_string()),
                error_description: None,
                message: "Error when inserting user".to_string(),
            }),
            Self::DeleteUser(err) => HttpResponse::InternalServerError().json(ErrorMessage {
                error: Option::from(err.to_string()),
                error_description: None,
                message: "Error when deleting user".to_string(),
            }),
            Self::GeneralError(err) => HttpResponse::InternalServerError().json(ErrorMessage {
                error: Option::from(err.to_string()),
                error_description: None,
                message: "General error".to_string(),
            }),
        }
    }
}
