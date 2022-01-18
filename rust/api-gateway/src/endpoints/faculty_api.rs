use crate::{
    constants::{urls, faculty_const},
    models::faculty::{FacultyDTO},
    utils::client_response,
};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Path};
use awc;


// POST /faculties
pub async fn create_faculty(faculty_dto: web::Json<FacultyDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &urls::FACULTY_SERVICE, &faculty_const::FACUTIES);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.post(url)
                .header("Authorization", token.unwrap())
                .send_json(&faculty_dto.into_inner())
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

// GET /faculties/{id}
pub async fn get_faculty(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", &urls::FACULTY_SERVICE, &faculty_const::FACUTIES_ID, id.into_inner());
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

// GET /faculties
pub async fn get_all_faculties(req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &urls::FACULTY_SERVICE, &faculty_const::FACUTIES);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    info!("Token:{}", token.unwrap());
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

// DELETE /faculties/{id}
pub async fn delete_faculty(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();

    let faculty_id = id.into_inner();

    let url = format!("{}{}{}", &urls::FACULTY_SERVICE, &faculty_const::FACUTIES_ID, faculty_id);
    info!("{}", url);
    
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.delete(url)
                .header("Authorization", token.unwrap())
                .send()
                .await;
    match resp {
        Ok(response) => {
            let client = awc::Client::new();
            let url = format!("{}{}{}", &urls::STAFF_SERVICE, &faculty_const::STAFF_FACULTY_ID, faculty_id);
            info!("{}", url);
            let token = req.headers().get("Authorization").unwrap().to_str().ok();
            let resp_faculty = client.delete(url)
                        .header("Authorization", token.unwrap())
                        .send()
                        .await;
            match resp_faculty {
                Ok(_) => {
                    return client_response::convert_to_http_response(response).await;
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

// UPDATE /faculties
pub async fn update_faculty(faculty_dto: web::Json<FacultyDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", &urls::FACULTY_SERVICE, &faculty_const::FACUTIES);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.put(url)
                .header("Authorization", token.unwrap())
                .send_json(&faculty_dto.into_inner())
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