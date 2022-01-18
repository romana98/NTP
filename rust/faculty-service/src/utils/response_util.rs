use std::env;
use actix_web::{error, Error, HttpResponse};
use crate::models::{shift::ShiftDTO, hard_constraints::HardConstraintDTO, faculty::FacultyDTO, staff_faculty::StaffFacultyDTO};

pub fn error_response(error: diesel::result::Error) -> Error {
    info!("   error_response");
    match error {
        diesel::result::Error::NotFound => error::ErrorNotFound(error.to_string()),
        _ => error::ErrorInternalServerError(error.to_string()),
    }
}

pub fn shift_created(shift: ShiftDTO) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = shift.id
            )
            .to_string(),
        )
        .json(shift)
}

pub fn shift_ok(shift: ShiftDTO) -> HttpResponse {
    HttpResponse::Ok()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = shift.id
            )
            .to_string(),
        )
        .json(shift)
}

pub fn hard_constraint_created(hard_constraint: HardConstraintDTO) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = hard_constraint.id
            )
            .to_string(),
        )
        .json(hard_constraint)
}

pub fn hard_constraint_ok(hard_constraint: HardConstraintDTO) -> HttpResponse {
    HttpResponse::Ok()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = hard_constraint.id
            )
            .to_string(),
        )
        .json(hard_constraint)
}

pub fn faculty_created(faculty: FacultyDTO) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = faculty.id
            )
            .to_string(),
        )
        .json(faculty)
}

pub fn faculty_ok(faculty: FacultyDTO) -> HttpResponse {
    HttpResponse::Ok()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = faculty.id
            )
            .to_string(),
        )
        .json(faculty)
}

pub fn staff_faculty_created(staff_faculty: StaffFacultyDTO) -> HttpResponse {
    HttpResponse::Created()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = staff_faculty.id
            )
            .to_string(),
        )
        .json(staff_faculty)
}

pub fn staff_faculty_ok(staff_faculty: StaffFacultyDTO) -> HttpResponse {
    HttpResponse::Ok()
        .header(
            "Location",
            format!(
                "{host}:{port}/api/staff/{id}",
                host = host(),
                port = port(),
                id = staff_faculty.id
            )
            .to_string(),
        )
        .json(staff_faculty)
}

fn host() -> String {
    env::var("ADDRESS").expect("ADDRESS must be set")
}

fn port() -> String {
    env::var("PORT").expect("PORT must be set")
}