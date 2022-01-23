use crate::{
    config::db::Pool,
    services::staff_service,
    utils::{response_util},
    enums::role
};
use actix_web::{error, Error, HttpResponse, Result};
use actix_web::web::{Data, Path};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};


// GET /schedule/{id} //id = faculty_id
pub async fn get_staff_by_faculty(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Get staff requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            staff_service::get_staff_by_faculty(id.into_inner(), &pool)
                .map(|lectures| {
                        HttpResponse::Ok().json(lectures)

                        
                    })
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}