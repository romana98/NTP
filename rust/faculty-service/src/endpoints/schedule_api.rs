use crate::{
    config::db::Pool,
    services::faculty_service,
    utils::response_util,
    enums::role
};
use actix_web::web::{Path};
use actix_web::{web, error, Error, HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// GET /schedule/{id}
pub async fn get_faculty(id: Path<i32>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting faculty requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            faculty_service::get_faculty_for_schedule(id.into_inner(), &pool)
                .map(|faculty| HttpResponse::Ok().json(faculty))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}