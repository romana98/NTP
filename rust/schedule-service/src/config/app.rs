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
                web::scope("/schedule")

                    .service(
                        web::resource("")
                            .route(web::post().to(schedule_api::generate_schedule))
                    )
                    .service(
                        web::resource("/by-staff/{id}")
                            .route(web::get().to(schedule_api::get_schedule_by_staff))
                    )   
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(schedule_api::get_schedule))
                    )
            )
    );
}