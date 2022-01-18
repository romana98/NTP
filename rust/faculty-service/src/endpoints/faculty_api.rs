use crate::{
    config::db::Pool,
    models::faculty::{FacultyDTO},
    services::faculty_service,
    utils::response_util,
    enums::role
};
use actix_web::web::{Data, Path};
use actix_web::{web, error, Error, HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// POST /faculties
pub async fn create_faculty(faculty_dto: web::Json<FacultyDTO>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Creating faculty requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let faculty_dto_inner = faculty_dto.into_inner();

            faculty_service::create_faculty(faculty_dto_inner, &pool)
            .map(|faculty| response_util::faculty_created(faculty))
            .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /faculties/{id}
pub async fn get_faculty(id: Path<i32>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting faculty requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            faculty_service::get_faculty(id.into_inner(), &pool)
                .map(|faculty| HttpResponse::Ok().json(faculty))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /faculties
pub async fn get_all_faculties(pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting all faculties requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            faculty_service::get_all_faculties(&pool)
                .map(|faculties| HttpResponse::Ok().json(faculties))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// DELETE /faculties/{id}
pub async fn delete_faculty(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Deleting faculty requested");
   
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            faculty_service::delete_faculty(id.into_inner(), &pool)
            .map(|_| HttpResponse::Ok().finish())
            .map_err(|error| error)
        
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// UPDATE /faculties
pub async fn update_faculty(faculty_dto: web::Json<FacultyDTO>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Update faculty requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let faculty_dto_inner = faculty_dto.into_inner();

            faculty_service::update_faculty(faculty_dto_inner, &pool)
                .map(|faculty| response_util::faculty_ok(faculty))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}