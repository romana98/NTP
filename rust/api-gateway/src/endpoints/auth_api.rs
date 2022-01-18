use crate::{
    constants::{urls, auth_const},
    models::auth::{LoginDTO},
    utils::client_response,
};
use actix_web::{web, HttpResponse};
use awc;

// POST /auth/log-in
pub async fn login(login_dto: web::Json<LoginDTO>) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &urls::AUTH_SERVICE, &auth_const::LOGIN);
    info!("{}", url);
    
    let resp = client.post(url).send_json(&login_dto.into_inner()).await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}