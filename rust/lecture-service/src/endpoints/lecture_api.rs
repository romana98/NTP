use crate::{
    config::db::Pool,
    models::lecture::{LectureDTO, IdsDTO},
    services::lecture_service,
    utils::response_util,
    enums::role
};
use actix_web::web::{Data, Path};
use actix_web::{web, error, Error, HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// POST /lectures
pub async fn create_lecture(lecture_dto: web::Json<LectureDTO>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Creating lecture requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let lec = lecture_dto.into_inner();

            lecture_service::create_lecture(lec, &pool)
            .map(|lecture| HttpResponse::Created().json(lecture))
            .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /lectures/{id}
pub async fn get_lecture(id: Path<i32>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting lecture requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            lecture_service::get_lecture(id.into_inner(), &pool)
                .map(|lecture| HttpResponse::Ok().json(lecture))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /lectures
pub async fn get_all_lectures(pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting lectures requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            lecture_service::get_all_lectures(&pool)
                .map(|lecture| HttpResponse::Ok().json(lecture))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /lectures/by-ids
pub async fn get_all_lectures_by_ids(ids_dto: web::Json<IdsDTO>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting lectures by faculty requested");
    
    match details.has_any_permission(&[&role::Role::Staff.to_string(), &role::Role::Admin.to_string()]) {
        true => {
            let ids = ids_dto.into_inner();

            lecture_service::get_all_lectures_by_ids(ids.ids, &pool)
                .map(|lecture| HttpResponse::Ok().json(lecture))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// DELETE /lectures/{id}
pub async fn delete_lecture(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Deleting lecture requested");
   
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            lecture_service::delete_lecture(id.into_inner(), &pool)
            .map(|_| HttpResponse::Ok().finish())
            .map_err(|error| error)
        
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// UPDATE /lectures
pub async fn update_lecture(lecture_dto: web::Json<LectureDTO>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Update lecture requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let lec = lecture_dto.into_inner();

            lecture_service::update_lecture(lec, &pool)
                .map(|lecture| HttpResponse::Ok().json(lecture))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}