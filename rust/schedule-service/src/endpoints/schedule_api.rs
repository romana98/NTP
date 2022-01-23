use crate::{
    config::db::Pool,
    models::{algorithm::{FacultySchedule, StaffSchedule, LectureSchedule, IdDTO}},
    constants::{urls},
    enums::role,
    algorithm::nsp,
    services::schedule_service
};
use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};
use actix_web::{web, HttpRequest, HttpResponse};
//use actix_web::web::{Path};
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
                                        Ok(res) => {return HttpResponse::Ok().json(res);},
                                        Err(error) => {return HttpResponse::BadRequest().body(error.to_string());}
                                    };
                                    //return HttpResponse::Ok().json(schedule);
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
