#[macro_use]
extern crate diesel;

use actix_web::{middleware, App, HttpServer};
use actix_rt;
use pretty_env_logger;
use log::*;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod rest;
mod dao;
mod contract;

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "actix_web=debug,diesel=debug");
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    // Setup database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("mysql://root:admin12345@localhost/actix-poc");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool");
    let bind = "0.0.0.0:8080";

    info!("Starting server at {}", &bind);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(rest::customer::get_customer)
            .service(rest::customer::insert)
            .service(rest::customer::update)
            .service(rest::customer::delete)
            .service(rest::customer::get_all)
    })
    .bind(&bind)?
    .run()
    .await
}
