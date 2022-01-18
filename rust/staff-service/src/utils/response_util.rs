use std::env;
use actix_web::{error, Error, HttpResponse};
use crate::models::{staff::StaffDTO, /*soft_constraint::SoftConstrainstDTO*/};

pub fn error_response(error: diesel::result::Error) -> Error {
    match error {
        diesel::result::Error::NotFound => error::ErrorNotFound(error.to_string()),
        _ => error::ErrorInternalServerError(error.to_string()),
    }
}

pub fn staff_created(staff: StaffDTO) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = staff.id
            )
            .to_string(),
        )
        .json(staff)
}

pub fn staff_ok(staff: StaffDTO) -> HttpResponse {
    HttpResponse::Ok()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = staff.id
            )
            .to_string(),
        )
        .json(staff)
}
/*
pub fn sc_ok(soft_constraint: SoftConstrainstDTO) -> HttpResponse {
    HttpResponse::Ok()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/soft-constraints/{id}",
                host = host(),
                port = port(),
                id = soft_constraint.id
            )
            .to_string(),
        )
        .json(soft_constraint)
}
*/

fn host() -> String {
    env::var("ADDRESS").expect("ADDRESS must be set")
}

fn port() -> String {
    env::var("PORT").expect("PORT must be set")
}