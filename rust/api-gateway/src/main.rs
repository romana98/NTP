#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_web;

extern crate dotenv;
extern crate derive_more;

use actix_web::{http, middleware, App, HttpServer};
use actix_cors::Cors;
use std::env;

mod constants;
mod config;
mod endpoints;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv::dotenv().unwrap();

    let address = env::var("ADDRESS").expect("ADDRESS not found.");
    let port = env::var("PORT").expect("PORT not found.");
    let app_url = format!("{}:{}", &address, &port);   

    let log = config::log::config_logging();
    log4rs::init_config(log).expect("Configuring logging");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allow_any_origin()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600))
            .wrap(middleware::Logger::default())
            .configure(config::app::config_services)
        })
    .bind(&app_url)?
    .run()
    .await
}
