#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;
extern crate r2d2;

use actix_web::{http, middleware, App, HttpServer};
use actix_cors::Cors;
use std::env;
use diesel::prelude::*;
use diesel::connection::Connection;
use crate::endpoints::check;

mod config;
mod endpoints;
mod models;
mod repository;
mod services;
mod utils;
mod enums;
mod schema;

embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv::dotenv().unwrap();

    let db_url= env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let address = env::var("ADDRESS").expect("ADDRESS not found.");
    let port = env::var("PORT").expect("PORT not found.");
    let app_url = format!("{}:{}", &address, &port);   

    let pool = config::db::init_pool();

    loop {
        let conn: Result<PgConnection, _> = Connection::establish(&db_url);
        if let Ok(conn) = conn {
            info!("Database connected");
            let mig = embedded_migrations::run(&conn);
            if let Err(_) = mig {
                info!("Puta");
            }
            break;
        }
    }

    let log = config::log::config_logging();
    log4rs::init_config(log).expect("Configuring logging");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default()
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                .allowed_header(http::header::CONTENT_TYPE)
                .max_age(3600))
            .wrap(middleware::Logger::default())
            .data(pool.clone())
            .service(check::healthcheck)
            .configure(config::app::config_services)
        })
    .bind(&app_url)?
    .run()
    .await
}
