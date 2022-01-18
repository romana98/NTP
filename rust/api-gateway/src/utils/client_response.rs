use crate::actix_web::HttpMessage;
use actix_web::{http::StatusCode, HttpResponse};
use awc;

pub type Argument = awc::ClientResponse<
    actix_web::dev::Decompress<
        actix_web::dev::Payload<
            std::pin::Pin<
                std::boxed::Box<
                    dyn futures_core::stream::Stream<
                        Item = std::result::Result<
                            actix_web::web::Bytes,
                            actix_web::error::PayloadError,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;

pub async fn convert_to_http_response(mut resp: Argument) -> HttpResponse {
    let body = resp.body().limit(20_000_000).await.unwrap();
    match resp.status() {
        StatusCode::OK => match resp.content_type() {
            "application/json" => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body);
            }
            _ => {
                return HttpResponse::Ok().body(body);
            }
        },
        StatusCode::NOT_FOUND => {
            return HttpResponse::NotFound().body(body);
        }
        StatusCode::METHOD_NOT_ALLOWED => {
            return HttpResponse::MethodNotAllowed().body(body);
        }
        StatusCode::FORBIDDEN => {
            return HttpResponse::Forbidden().body(body);
        }
        StatusCode::BAD_REQUEST => {
            return HttpResponse::BadRequest().body(body);
        }
        StatusCode::UNAUTHORIZED => {
            return HttpResponse::Unauthorized().body(body);
        }
        StatusCode::NO_CONTENT => {
            return HttpResponse::NoContent().finish();
        }
        StatusCode::CREATED => match resp.content_type() {
            "application/json" => {
                return HttpResponse::Created()
                    .content_type("application/json")
                    .body(body);
            }
            _ => {
                return HttpResponse::Created().body(body);
            }
        },
        _ => {
            return HttpResponse::InternalServerError().body(body);
        }
    }
}