use crate::{
    config::db::Pool,
    models::{
        staff::{StaffDTO}
    },
    services::staff_service,
    utils::{response_util, token_util},
    enums::role
};
use actix_web::{web, error, Error, HttpResponse, HttpRequest, Result};
use actix_web::web::{Data, Path};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};

// POST /staff
pub async fn create_staff(staff_dto: web::Json<StaffDTO>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Creating staff requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let s = staff_dto.into_inner();
    
            staff_service::create_staff(s, &pool)
                .map(|staff| response_util::staff_created(staff))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /staff/{id}
pub async fn get_staff(id: Path<i32>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting staff requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            staff_service::get_staff(id.into_inner(), &pool)
                .map(|staff| HttpResponse::Ok().json(staff))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /staff
pub async fn get_all_staff(pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting all staff requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            staff_service::get_all_staff(&pool)
                .map(|staff| HttpResponse::Ok().json(staff))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /staff/lectures
pub async fn get_staff_lectures(pool: web::Data<Pool>, http_request: HttpRequest, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting lectures_staff requested");

    match details.has_permission(&role::Role::Staff.to_string()) {
        true => {

            let id = token_util::get_id(http_request).unwrap();    

            staff_service::get_staff_lectures(id, &pool)
                .map(|lectures| {
                        HttpResponse::Ok().json(lectures)

                        
                    })
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// UPDATE /staff
pub async fn update_staff(staff_dto: web::Json<StaffDTO>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Update staff requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let staff = staff_dto.into_inner();

            staff_service::update_staff(staff, &pool)
                .map(|staff| response_util::staff_ok(staff))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// DELETE /staff/{id}
pub async fn delete_staff(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Deleting lecture requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            staff_service::delete_staff(id.into_inner(), &pool)
                .map(|_| HttpResponse::Ok().finish())
                .map_err(|error| error)
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// DELETE /staff/faculty/{id} //id = faculty_id
pub async fn delete_staff_by_faculty(id: Path<i32>, pool: Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Deleting lecture requested");

    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            staff_service::delete_staff_by_faculty(id.into_inner(), &pool)
                .map(|_| HttpResponse::Ok().finish())
                .map_err(|error| error)
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}