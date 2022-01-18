use actix_web::HttpResponse;

#[get("/healthcheck")]
fn healthcheck() -> HttpResponse {
    HttpResponse::Ok()
        .body("OK!".to_string())
}