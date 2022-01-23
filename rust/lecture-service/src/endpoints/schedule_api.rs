use crate::{
    config::db::Pool,
    services::lecture_service,
    utils::response_util,
    enums::role
};
use actix_web::{web, error, Error, HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// GET /schedule/
pub async fn get_all_lectures_by_ids(ids: web::Json<Vec<i32>>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting lectures by faculty requested");
    
    match details.has_any_permission(&[&role::Role::Staff.to_string(), &role::Role::Admin.to_string()]) {
        true => {
            lecture_service::get_all_lectures_by_ids_schedule(ids.into_inner(), &pool)
                .map(|lecture| HttpResponse::Ok().json(lecture))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}