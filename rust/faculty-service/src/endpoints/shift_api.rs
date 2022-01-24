use crate::{
    config::db::Pool,
    models::shift::{ShiftDTO},
    services::shift_service,
    utils::response_util,
    enums::role
};
use actix_web::web::{Data, Path};
use actix_web::{web, error, Error, HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// POST /shifts
pub async fn create_shift(shift_dto: web::Json<ShiftDTO>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Creating shift requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let shift_dto_inner = shift_dto.into_inner();

            shift_service::create_shift(shift_dto_inner, &pool)
            .map(|shift|  HttpResponse::Created().json(shift))
            .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /shifts/{id}
pub async fn get_shift(id: Path<i32>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting shift requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            shift_service::get_shift(id.into_inner(), &pool)
                .map(|shift| HttpResponse::Ok().json(shift))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /shifts
pub async fn get_all_shifts(pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting shift requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            shift_service::get_all_shifts(&pool)
                .map(|shifts| HttpResponse::Ok().json(shifts))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// DELETE /shifts/{id}
pub async fn delete_shift(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Deleting shift requested");
   
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            shift_service::delete_shift(id.into_inner(), &pool)
            .map(|_| HttpResponse::Ok().finish())
            .map_err(|error| error)
        
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// UPDATE /shifts
pub async fn update_shift(shift_dto: web::Json<ShiftDTO>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Update shift requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let shift_dto_inner = shift_dto.into_inner();

            shift_service::update_shift(shift_dto_inner, &pool)
                .map(|shift| HttpResponse::Ok().json(shift))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}