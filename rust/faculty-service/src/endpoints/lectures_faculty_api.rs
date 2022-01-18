use crate::{
    config::db::Pool,
    services::lectures_faculty_service,
    enums::role
};
use actix_web::web::{Data, Path};
use actix_web::{error, Error, HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};


// DELETE /lectures/{id}
pub async fn delete_lectures_faculty(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Deleting faculty requested");
   
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            lectures_faculty_service::delete_lectures_faculty(id.into_inner(), &pool)
            .map(|_| HttpResponse::Ok().finish())
            .map_err(|error| error)
        
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}