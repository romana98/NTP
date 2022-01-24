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
                web::scope("/staff")

                    .service(
                        web::resource("")
                            .route(web::post().to(staff_api::create_staff))
                            .route(web::put().to(staff_api::update_staff))
                            .route(web::get().to(staff_api::get_all_staff))
                    )
                    .service(
                        web::resource("/lectures")
                            .route(web::get().to(staff_api::get_staff_lectures))
                    )
                    .service(
                        web::resource("/id")
                            .route(web::get().to(staff_api::get_staff_id))
                    )      
                    .service(
                        web::resource("/faculty/{id}")
                        .route(web::delete().to(staff_api::delete_staff_by_faculty))
                    )   
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(staff_api::get_staff))
                            .route(web::delete().to(staff_api::delete_staff))
                    )
            )
            .service(
                web::scope("/soft-constraints")
                    .service(
                        web::resource("")
                            .route(web::put().to(soft_constraints_api::update_soft_constraints))
                            .route(web::get().to(soft_constraints_api::get_soft_constraints))
                    )
            )
            .service(
                web::scope("/lectures")
                    .service(
                        web::resource("/{id}")
                            .route(web::delete().to(lectures_staff_api::delete_lectures_staff))
                    )
            )
            .service(
                web::scope("/schedule")
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(schedule_api::get_staff_by_faculty))
                    )
            )
    );
}