use actix_web::{web, App, HttpServer};
use actix_rt;
use pretty_env_logger;
use log::*;

mod rest;
mod contract; 

#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    pretty_env_logger::init();
    info!("Starting server...");
    HttpServer::new(||{
        App::new()
            .service(
                web::scope("/rest")
                    .route("", web::get().to(rest::v1::index::index))
            )
            .service(
                web::scope("/customer")
                    .route("", web::post().to(rest::v1::customer::save))
                    .route("", web::get().to(rest::v1::customer::list))
                    .route("/query", web::get().to(rest::v1::customer::query))
                    .route("/{id}", web::get().to(rest::v1::customer::get_by_id))
                    .route("/{id}", web::put().to(rest::v1::customer::update))
                    .route("/{id}", web::delete().to(rest::v1::customer::delete))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
