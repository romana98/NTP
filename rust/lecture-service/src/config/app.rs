use crate::endpoints::*;
use crate::utils::token_util;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(token_util::validator);

    cfg.service (
        web::scope("/")
            .wrap(auth)
            .service(
                web::scope("/lectures")
                    .service(
                        web::resource("")
                            .route(web::post().to(lecture_api::create_lecture))
                            .route(web::put().to(lecture_api::update_lecture))
                            .route(web::get().to(lecture_api::get_all_lectures))
                    )
                    .service(
                        web::resource("/by-ids")
                            .route(web::post().to(lecture_api::get_all_lectures_by_ids))
                    )
                    .service(
                        web::resource("/by-staff")
                            .route(web::get().to(lecture_api::get_all_lectures))
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(lecture_api::get_lecture))
                            .route(web::delete().to(lecture_api::delete_lecture))
                    )
            )
            .service(
                web::scope("/schedule")
                    .service(
                        web::resource("")
                            .route(web::post().to(schedule_api::get_all_lectures_by_ids))
                    )
            )   
    );
}