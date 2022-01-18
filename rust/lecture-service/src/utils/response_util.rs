use actix_web::{error, Error, HttpResponse};
use std::env;
use crate::models::lecture::LectureDTO;

pub fn error_response(error: diesel::result::Error) -> Error {
    match error {
        diesel::result::Error::NotFound => error::ErrorNotFound(error.to_string()),
        _ => error::ErrorInternalServerError(error.to_string()),
    }
}

pub fn lecture_created(lecture: LectureDTO) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/lectures/{id}",
                host = host(),
                port = port(),
                id = lecture.id
            )
            .to_string(),
        )
        .json(lecture)
}

pub fn lecture_ok(lecture: LectureDTO) -> HttpResponse {
    HttpResponse::Ok()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/lectures/{id}",
                host = host(),
                port = port(),
                id = lecture.id
            )
            .to_string(),
        )
        .json(lecture)
}

fn host() -> String {
    env::var("ADDRESS").expect("ADDRESS must be set")
}

fn port() -> String {
    env::var("PORT").expect("PORT must be set")
}