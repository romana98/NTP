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
                web::scope("/faculties")
                .service(
                    web::resource("")
                        .route(web::post().to(faculty_api::create_faculty))
                        .route(web::put().to(faculty_api::update_faculty))
                        .route(web::get().to(faculty_api::get_all_faculties))
                )
                .service(
                    web::resource("/{id}")
                        .route(web::get().to(faculty_api::get_faculty))
                        .route(web::delete().to(faculty_api::delete_faculty))
                )
            )
            .service(
                web::scope("/shifts")
                    .service(
                        web::resource("")
                            .route(web::post().to(shift_api::create_shift))
                            .route(web::put().to(shift_api::update_shift))
                            .route(web::get().to(shift_api::get_all_shifts))
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(shift_api::get_shift))
                            .route(web::delete().to(shift_api::delete_shift))
                    )
            )
            .service(
                web::scope("/hard-constraints")
                .service(
                    web::resource("")
                        .route(web::post().to(hard_constraints_api::create_hard_constraint))
                        .route(web::put().to(hard_constraints_api::update_hard_constraint))
                        .route(web::get().to(hard_constraints_api::get_all_hard_constraints))
                )
                .service(
                    web::resource("/{id}")
                        .route(web::get().to(hard_constraints_api::get_hard_constraint))
                        .route(web::delete().to(hard_constraints_api::delete_hard_constraint))
                )
            )
            .service(
                web::scope("/lectures")
                    .service(
                        web::resource("/{id}")
                            .route(web::delete().to(lectures_faculty_api::delete_lectures_faculty))
                    )
            )
            .service(
                web::scope("/staff")
                    .service(
                        web::resource("")
                            .route(web::post().to(staff_faculty_api::create_staff_faculty))
                            .route(web::put().to(staff_faculty_api::update_staff_faculty))
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::delete().to(staff_faculty_api::delete_staff_faculty))
                    )
            )
    );
}