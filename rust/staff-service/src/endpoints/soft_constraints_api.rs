use crate::{
    config::db::Pool,
    models::{
        soft_constraint::{SoftConstraintsDTO}
    },
    services::soft_constraints_service,
    utils::{response_util, token_util},
    enums::role
};

use actix_web::{web, error, Error, HttpResponse, HttpRequest, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// GET /soft-constraints/{id}
pub async fn get_soft_constraints(pool: web::Data<Pool>, http_request: HttpRequest, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting soft constraints requested");

    match details.has_permission(&role::Role::Staff.to_string()) {
        true => {

            let id = token_util::get_id(http_request).unwrap();      

            soft_constraints_service::get_soft_constraints(id, &pool)
                .map(|staff| HttpResponse::Ok().json(staff))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// UPDATE /soft-constraints
pub async fn update_soft_constraints(sc_dto: web::Json<SoftConstraintsDTO>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Update soft constraints requested");

    match details.has_permission(&role::Role::Staff.to_string()) {
        true => {

            let sc = sc_dto.into_inner();    

            soft_constraints_service::update_soft_constraints(sc, &pool)
                .map(|staff| HttpResponse::Ok().json(staff))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}