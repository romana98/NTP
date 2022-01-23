use crate::{
    constants::{urls, schedule_const},
    models::schedule::{IdDTO},
    utils::client_response,
};
use actix_web::{web, HttpRequest, HttpResponse};
//use actix_web::web::{Path};
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
