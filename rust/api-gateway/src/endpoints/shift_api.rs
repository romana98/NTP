use crate::{
    constants::{urls, faculty_const},
    models::shift::{ShiftDTO},
    utils::client_response,
};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Path};
use awc;


// POST /shifts
pub async fn create_shift(shift_dto: web::Json<ShiftDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::FACULTY_SERVICE, &faculty_const::SHIFTS);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.post(url)
                .header("Authorization", token.unwrap())
                .send_json(&shift_dto.into_inner())
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

// GET /shifts/{id}
pub async fn get_shift(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", *urls::FACULTY_SERVICE, &faculty_const::SHIFTS_ID, id.into_inner());
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
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

// GET /shifts
pub async fn get_all_shifts(req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::FACULTY_SERVICE, &faculty_const::SHIFTS);
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
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

// DELETE /shifts/{id}
pub async fn delete_shift(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", *urls::FACULTY_SERVICE, &faculty_const::SHIFTS_ID, id.into_inner());
    info!("{}", url);
    
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.delete(url)
                .header("Authorization", token.unwrap())
                .send()
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

// UPDATE /shifts
pub async fn update_shift(shift_dto: web::Json<ShiftDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::FACULTY_SERVICE, &faculty_const::SHIFTS);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.put(url)
                .header("Authorization", token.unwrap())
                .send_json(&shift_dto.into_inner())
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