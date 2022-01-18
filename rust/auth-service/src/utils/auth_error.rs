use std::convert::From;
use derive_more::Display;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::{DatabaseErrorKind, Error as DBError};

#[derive(Clone, Debug, Display)]
pub enum AuthError {
    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),

    #[display(fmt = "GenericError: {}", _0)]
    GenericError(String),

    #[display(fmt = "UniqueViolation: {}", _0)]
    UniqueViolation(String),

    //#[display(fmt = "ProcessError: {}", _0)]
    //ProcessError(String),
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::NotFound(ref message) => HttpResponse::NotFound().json(message),

            AuthError::GenericError(ref message) => HttpResponse::BadRequest().json(message),

            AuthError::UniqueViolation(ref message) => HttpResponse::BadRequest().json(message),

            /*AuthError::ProcessError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }*/
        }
    }
}

impl From<DBError> for AuthError {
    fn from(error: DBError) -> AuthError {
        match error {
            DBError::DatabaseError(kind, info) => {
                let message = info.details().unwrap_or_else(|| info.message()).to_string();

                match kind {
                    DatabaseErrorKind::UniqueViolation => AuthError::UniqueViolation(message),
                    _ => AuthError::GenericError(message),
                }
            }
            _ => AuthError::GenericError(String::from("Some database error occured")),
        }
    }
}