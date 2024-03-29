use actix_web::{error, Error};

pub fn error_response(error: diesel::result::Error) -> Error {
    info!("   error_response");
    match error {
        diesel::result::Error::NotFound => error::ErrorNotFound(error.to_string()),
        _ => error::ErrorInternalServerError(error.to_string()),
    }
}