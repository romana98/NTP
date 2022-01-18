use crate::{
    config::db::Pool,
    models::auth::{LoginDTO},
    services::auth_service,
    utils::auth_error::{AuthError},
};
use actix_web::{web, HttpResponse, Result};

// POST auth/log-in
pub async fn login(login_dto: web::Json<LoginDTO>, pool: web::Data<Pool>) -> Result<HttpResponse, AuthError> {
    info!("Login requested");

    match auth_service::login(login_dto.into_inner(), &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(token_res)),
        Err(err) => Err(err),
    }
}