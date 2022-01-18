use crate::{
    constants::{urls, staff_const},
    models::soft_constraints::{SoftConstrainstDTO},
    utils::client_response,
};
use actix_web::{web, HttpRequest, HttpResponse};
use awc;

// UPDATE /soft-constraints
pub async fn update_soft_constraints(sc_dto: web::Json<SoftConstrainstDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &urls::STAFF_SERVICE, &staff_const::SC);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.put(url)
                .header("Authorization", token.unwrap())
                .send_json(&sc_dto.into_inner())
                .await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

// GET /soft-constraints
pub async fn get_soft_constraints(req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &urls::STAFF_SERVICE, &staff_const::SC);
    info!("{}", url);
    
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.get(url)
                .header("Authorization", token.unwrap())
                .send()
                .await;
    match resp {
        Ok(response) => {
            return client_response::convert_to_http_response(response).await;
        }
        Err(error) => {
            return HttpResponse::BadRequest().body(error.to_string());
        }
    }
}
