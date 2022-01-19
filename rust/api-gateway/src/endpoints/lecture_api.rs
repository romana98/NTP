use crate::{
    constants::{urls, lecture_const},
    models::lecture::{LectureDTO, IdsDTO},
    utils::client_response,
};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Path};
use awc;


// POST /lectures
pub async fn create_lecture(lecture_dto: web::Json<LectureDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::LECTURE_SERVICE, &lecture_const::LECTURES);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.post(url)
                .header("Authorization", token.unwrap())
                .send_json(&lecture_dto.into_inner())
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

// GET /lectures/{id}
pub async fn get_lecture(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", *urls::LECTURE_SERVICE, &lecture_const::LECTURES_ID, id.into_inner());
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

// GET /lectures
pub async fn get_all_lectures(req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::LECTURE_SERVICE, &lecture_const::LECTURES);
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

// POST /lectures/by-ids
pub async fn get_all_lectures_by_ids(ids_vec: web::Json<Vec<String>>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}",*urls::LECTURE_SERVICE, &lecture_const::LECTURES_IDS);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();

    let ids_dto = IdsDTO{ids: ids_vec.into_inner()};

    let resp = client.post(url)
                .header("Authorization", token.unwrap())
                .send_json(&ids_dto)
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

// DELETE /lectures/{id}
pub async fn delete_lecture(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let id_lecture = id.into_inner();
    let url = format!("{}{}{}", *urls::LECTURE_SERVICE, &lecture_const::LECTURES_ID, id_lecture);
    info!("{}", url);
    
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.delete(url)
                .header("Authorization", token.unwrap())
                .send()
                .await;
    match resp {
        Ok(response) => {
            let url_staff = format!("{}{}{}", *urls::STAFF_SERVICE, &lecture_const::LECTURES_ID, id_lecture);
            let resp_staff = client.delete(url_staff)
                        .header("Authorization", token.unwrap())
                        .send()
                        .await;
            
            match resp_staff {
                Ok(_) => {
                    let url_faculty = format!("{}{}{}", *urls::FACULTY_SERVICE, &lecture_const::LECTURES_ID, id_lecture);
                    let resp_faculty = client.delete(url_faculty)
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
        Err(error) => {
            return HttpResponse::InternalServerError().body(error.to_string());
        }
    }
}

// UPDATE /lectures
pub async fn update_lecture(lecture_dto: web::Json<LectureDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::LECTURE_SERVICE, &lecture_const::LECTURES);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.put(url)
                .header("Authorization", token.unwrap())
                .send_json(&lecture_dto.into_inner())
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