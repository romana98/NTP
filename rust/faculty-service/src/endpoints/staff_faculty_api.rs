use crate::{
    config::db::Pool,
    services::staff_faculty_service,
    models::staff_faculty::StaffFacultyDTO,
    enums::role,
    utils::response_util
};
use actix_web::web::{Data, Path};
use actix_web::{web, error, Error, HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// POST /staff
pub async fn create_staff_faculty(staff_faculty_dto: web::Json<StaffFacultyDTO>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Creating staff_faculty requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let staff_faculty_dto_inner = staff_faculty_dto.into_inner();

            staff_faculty_service::create_staff_faculty(staff_faculty_dto_inner, &pool)
            .map(|staff_faculty| HttpResponse::Created().json(staff_faculty))
            .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// UPDATE /staff
pub async fn update_staff_faculty(staff_faculty_dto: web::Json<StaffFacultyDTO>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Update staff_faculty requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let staff_faculty_dto_inner = staff_faculty_dto.into_inner();

            staff_faculty_service::update_staff_faculty(staff_faculty_dto_inner, &pool)
                .map(|staff_faculty|  HttpResponse::Ok().json(staff_faculty))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// DELETE /staff/{id}
pub async fn delete_staff_faculty(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Deleting staff_faculty requested");
   
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            staff_faculty_service::delete_staff_faculty(id.into_inner(), &pool)
            .map(|_| HttpResponse::Ok().finish())
            .map_err(|error| error)
        
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}