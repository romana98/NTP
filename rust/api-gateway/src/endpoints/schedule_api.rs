use crate::{
    constants::{urls, schedule_const},
    models::schedule::{IdDTO, IdStaffDTO},
    utils::client_response,
};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Path};
use awc;

// POST /schedule
pub async fn generate_schedule(id: web::Json<IdDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::SCHEDULE_SERVICE, &schedule_const::SCHEDULE);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.post(url)
                .header("Authorization", token.unwrap())
                .send_json(&id.into_inner())
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

// GET /schedule/{id}
pub async fn get_schedule(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", *urls::SCHEDULE_SERVICE, &schedule_const::SCHEDULE_ID, id.into_inner());
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

// GET /schedule/by-staff
pub async fn get_schedule_by_staff(req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::STAFF_SERVICE, &schedule_const::STAFF_BY_ID);
    info!("{}", url);
    
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.get(url)
                .header("Authorization", token.unwrap())
                .send()
                .await;
    match resp {
        Ok(mut response) => {
            let staff_id = &response.json::<IdStaffDTO>().await.unwrap();
            let client = awc::Client::new();
            let url = format!("{}{}{}", *urls::SCHEDULE_SERVICE, &schedule_const::SCHEDULE_BY_STAFF, staff_id.id);
            info!("{}", url);
            
            let token = req.headers().get("Authorization").unwrap().to_str().ok();
            let resp_schedule = client.get(url)
                        .header("Authorization", token.unwrap())
                        .send()
                        .await;
            match resp_schedule {
                Ok(response_schedule) => {
                    return client_response::convert_to_http_response(response_schedule).await;
                }
                Err(error) => {
                    return HttpResponse::InternalServerError().body(error.to_string());
                }
            }
        }
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

