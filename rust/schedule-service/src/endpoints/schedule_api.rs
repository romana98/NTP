use crate::{
    config::db::Pool,
    models::{algorithm::{FacultySchedule, StaffSchedule, LectureSchedule, IdDTO, FacultyDTO}},
    constants::{urls},
    enums::role,
    algorithm::nsp,
    services::schedule_service,
    utils::response_util
};
use actix_web::{web, error, Error, HttpRequest,  HttpResponse, Result};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};
use actix_web::web::{Path};
use awc;



// POST /schedule/
pub async fn generate_schedule(id: web::Json<IdDTO>, pool: web::Data<Pool>, req: HttpRequest, details: AuthDetails) -> HttpResponse {
    info!("   Creating faculty requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            let client = awc::Client::new();
            let faculty_id = id.into_inner();
            let url = format!("{}{}{}", *urls::FACULTY_SERVICE, &urls::SCHEDULE_ID, faculty_id.id);
            info!("{}", url);
            
            let token = req.headers().get("Authorization").unwrap().to_str().ok();
            let resp = client.get(url)
                        .header("Authorization", token.unwrap())
                        .send()
                        .await;
            match resp {
                Ok(mut response) => {
                    let faculty = &response.json::<FacultySchedule>().await.unwrap();

                    let url_staff = format!("{}{}{}", *urls::STAFF_SERVICE, &urls::SCHEDULE_ID, faculty_id.id);
                    let resp_staff = client.get(url_staff)
                                .header("Authorization", token.unwrap())
                                .send()
                                .await;
                    match resp_staff {
                        Ok(mut response_staff) => {
                            let staff = &response_staff.json::<Vec<StaffSchedule>>().await.unwrap();

                            let url_lecture = format!("{}{}", *urls::LECTURE_SERVICE, &urls::SCHEDULE);
                            let resp_lecture = client.post(url_lecture)
                                        .header("Authorization", token.unwrap())
                                        .send_json(&faculty.lectures)
                                        .await;
                            match resp_lecture {
                                Ok(mut response_lecture) => {
                                    let lectures = &response_lecture.json::<Vec<LectureSchedule>>().await.unwrap();
                                    
                                    let schedule = nsp::start(faculty, staff, lectures);
                                    let service_result = schedule_service::save_schedule(schedule, &pool);

                                    match service_result{
                                        Ok(res) => {
                                            let url_faculty_schedule = format!("{}{}", *urls::FACULTY_SERVICE, &urls::SCHEDULE);
                                            let id_dto = FacultyDTO{faculty_id: faculty.id, schedule_id: res.schedule_id};
                                            
                                            let resp_faculty_schedule = client.put(url_faculty_schedule)
                                                        .header("Authorization", token.unwrap())
                                                        .send_json(&id_dto)
                                                        .await;
                                                        
                                            match resp_faculty_schedule{
                                                Ok(_) => {
                                                    schedule_service::delete_schedule(faculty.schedule_id, &pool).unwrap();
                                                    return HttpResponse::Ok().json(res.schedule);},
                                                Err(error) => {
                                                    return HttpResponse::BadRequest().body(error.to_string());
                                                }
                                            } 
                                        },
                                        Err(error) => {return HttpResponse::BadRequest().body(error.to_string());}
                                    };
                                }
                                Err(error) => {
                                    return HttpResponse::BadRequest().body(error.to_string());
                                }
                            }
                        }
                        Err(error) => {
                            return HttpResponse::BadRequest().body(error.to_string());
                        }
                    }
                }
                Err(error) => {
                    return HttpResponse::BadRequest().body(error.to_string());
                }
            }
        },
        false => HttpResponse::Unauthorized().body("Access denied".to_string())
    }
}

// GET /schedule/{id}
pub async fn get_schedule(id: Path<i32>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting schedule requested");
    
    match details.has_permission(&role::Role::Admin.to_string()) {
        true => {
            schedule_service::get_schedule(id.into_inner(), &pool)
                .map(|faculty| HttpResponse::Ok().json(faculty))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}

// GET /schedule/by-staff/{id}
pub async fn get_schedule_by_staff(id: Path<i32>, pool: web::Data<Pool>, details: AuthDetails) -> Result<HttpResponse, Error> {
    info!("   Getting schedule requested");
    
    match details.has_permission(&role::Role::Staff.to_string()) {
        true => {
            schedule_service::get_schedule_by_staff(id.into_inner(), &pool)
                .map(|faculty| HttpResponse::Ok().json(faculty))
                .map_err(|error| response_util::error_response(error))
        },
        false => Err(error::ErrorForbidden("Access denied")),
    }
}