use crate::{
    config::db::Pool,
    models::hard_constraints::{HardConstraintDTO},
    services::hard_constraints_service,
    utils::response_util,
    enums::role
};
use actix_web::web::{Data, Path};
use actix_web::{web, error, Error, HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// POST /hard_constraints
pub async fn create_hard_constraint(hard_constraint_dto: web::Json<HardConstraintDTO>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Creating hard_constraint requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let hard_constraint_dto_inner = hard_constraint_dto.into_inner();

            hard_constraints_service::create_hard_constraint(hard_constraint_dto_inner, &pool)
            .map(|hard_constraint| HttpResponse::Created().json(hard_constraint))
            .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /hard_constraints/{id}
pub async fn get_hard_constraint(id: Path<i32>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting hard_constraint requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            hard_constraints_service::get_hard_constraint(id.into_inner(), &pool)
                .map(|hard_constraint| HttpResponse::Ok().json(hard_constraint))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /hard_constraint
pub async fn get_all_hard_constraints(pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting hard_constraint requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            hard_constraints_service::get_all_hard_constraints(&pool)
                .map(|hard_constraints| HttpResponse::Ok().json(hard_constraints))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// DELETE /hard_constraints/{id}
pub async fn delete_hard_constraint(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Deleting hard_constraint requested");
   
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            hard_constraints_service::delete_hard_constraint(id.into_inner(), &pool)
            .map(|_| HttpResponse::Ok().finish())
            .map_err(|error| error)
        
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// UPDATE /hard_constraints
pub async fn update_hard_constraint(hard_constraint_dto: web::Json<HardConstraintDTO>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Update hard_constraint requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let hard_constraint_dto_inner = hard_constraint_dto.into_inner();

            hard_constraints_service::update_hard_constraint(hard_constraint_dto_inner, &pool)
                .map(|hard_constraint| HttpResponse::Ok().json(hard_constraint))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}