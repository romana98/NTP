use crate::{
    constants::{urls, staff_const, lecture_const},
    models::{staff::{StaffDTO}, lecture::{IdsDTO}, faculty::{StaffFacultyDTO}},
    utils::client_response,
};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::web::{Path};
use awc;

// POST /staff
pub async fn create_staff(staff_dto: web::Json<StaffDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::STAFF_SERVICE, &staff_const::STAFF);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.post(url)
                .header("Authorization", token.unwrap())
                .send_json(&staff_dto.into_inner())
                .await;
    match resp {
        Ok(mut response) => {
            let staff = &response.json::<StaffDTO>().await.unwrap();
            let staff_faculty = StaffFacultyDTO{
                id: 0,
                staff_id: staff.id.parse::<i32>().unwrap(),
                faculty_id: staff.faculty.parse::<i32>().unwrap()
            };
            
            let client = awc::Client::new();
            let url = format!("{}{}", *urls::FACULTY_SERVICE, &staff_const::STAFF);
            info!("{}", url);
            let token = req.headers().get("Authorization").unwrap().to_str().ok();
            let resp_faculty = client.post(url)
                        .header("Authorization", token.unwrap())
                        .send_json(&staff_faculty)
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

// GET /staff/{id}
pub async fn get_staff(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}{}", *urls::STAFF_SERVICE, &staff_const::STAFF_ID, id.into_inner());
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

// GET /staff
pub async fn get_all_staff(req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::STAFF_SERVICE, &staff_const::STAFF);
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

// POST /lectures/by-staff
pub async fn get_all_lectures_by_staff(req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url_staff = format!("{}{}", *urls::STAFF_SERVICE, &staff_const::STAFF_LECTURES);
    info!("{}", url_staff);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.get(url_staff)
                .header("Authorization", token.unwrap())
                .send()
                .await;
    match resp {
        Ok(mut response) => {
            let ids = &response.json::<IdsDTO>().await.unwrap();
            
            let client = awc::Client::new();
            let url = format!("{}{}", *urls::LECTURE_SERVICE, &lecture_const::LECTURES_IDS);
            info!("{}", url);
            let token = req.headers().get("Authorization").unwrap().to_str().ok();
            let resp_lecture = client.post(url)
                        .header("Authorization", token.unwrap())
                        .send_json(&ids)
                        .await;
            match resp_lecture {
                Ok(response_lecture) => {
                    return client_response::convert_to_http_response(response_lecture).await;
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


// UPDATE /staff
pub async fn update_staff(staff_dto: web::Json<StaffDTO>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let url = format!("{}{}", *urls::STAFF_SERVICE, &staff_const::STAFF);
    info!("{}", url);

    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.put(url)
                .header("Authorization", token.unwrap())
                .send_json(&staff_dto.into_inner())
                .await;
    match resp {
        Ok(mut response) => {
            let staff = &response.json::<StaffDTO>().await.unwrap();
            let staff_faculty = StaffFacultyDTO{
                id: 0,
                staff_id: staff.id.parse::<i32>().unwrap(),
                faculty_id: staff.faculty.parse::<i32>().unwrap()
            };
            
            let client = awc::Client::new();
            let url = format!("{}{}", *urls::FACULTY_SERVICE, &staff_const::STAFF);
            info!("{}", url);
            let token = req.headers().get("Authorization").unwrap().to_str().ok();
            let resp_faculty = client.put(url)
                        .header("Authorization", token.unwrap())
                        .send_json(&staff_faculty)
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

// DELETE /staff/{id}
pub async fn delete_staff(id: Path<i32>, req: HttpRequest) -> HttpResponse {
    let client = awc::Client::new();
    let staff_id = id.into_inner();
    let url = format!("{}{}{}", *urls::STAFF_SERVICE, &staff_const::STAFF_ID, staff_id);
    info!("{}", url);
    
    let token = req.headers().get("Authorization").unwrap().to_str().ok();
    let resp = client.delete(url)
                .header("Authorization", token.unwrap())
                .send()
                .await;
    match resp {
        Ok(response) => {
            let url_faculty = format!("{}{}{}", *urls::FACULTY_SERVICE, &staff_const::STAFF_ID, staff_id);
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
