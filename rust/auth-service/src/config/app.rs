
use crate::endpoints::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");

    cfg.service (
        web::scope("/") 
            .service(check::healthcheck)
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/log-in")
                            .route(web::post().to(auth_api::login))
                    )   
            )
    );
}