#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_web;


extern crate dotenv;
extern crate r2d2;
extern crate derive_more;
extern crate actix_web_grants; //::proc_macro::{has_any_role, has_permissions};

use actix_web::{http, middleware, App, HttpServer};
use actix_cors::Cors;
use std::env;

mod config;
mod endpoints;
mod enums;
mod models;
mod services;
mod utils;
mod repository;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv::dotenv().unwrap();

    let address = env::var("ADDRESS").expect("ADDRESS not found.");
    let port = env::var("PORT").expect("PORT not found.");
    let app_url = format!("{}:{}", &address, &port);   

    let pool = config::db::init_pool();

    let log = config::log::config_logging();
    log4rs::init_config(log).expect("Configuring logging");

    HttpServer::new(move || {
        info!("Starting rating microservice");
        App::new()
            .wrap(Cors::default()
                .allowed_methods(vec!["POST"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600))
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .configure(config::app::config_services)
        })
    .bind(&app_url)?
    .run()
    .await
}