use actix_web::{get, post, put, delete, web, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

use crate::dao;
use crate::contract;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/rest/customer")]
async fn get_all(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let result = web::block(move || dao::customer::get_all(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/rest/customer/{id}")]
async fn get_customer(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {

    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let customer = web::block(move || dao::customer::find_customer_by_id(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish();
        })?;

    if let Some(customer) = customer {
        Ok(HttpResponse::Ok().json(customer))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No customer found with uid: {}", user_uid));
        Ok(res)
    }
}

#[post("/rest/customer")]
async fn insert(
    pool: web::Data<DbPool>,
    form: web::Json<contract::customer::NewCustomer>
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Couldn't get db connection from pool");

    let customer = contract::customer::Customer {
        id: Uuid::new_v4().to_string(),
        document: form.document.clone(),
        name: form.name.clone(),
        second_name: form.second_name.clone(),
        person_type: form.person_type.clone(),
        device_id: form.device_id.clone(),
    };

    let user = web::block(move || dao::customer::insert(customer, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/rest/customer/{id}")]
async fn update_customer(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
    form: web::Json<contract::customer::NewCustomer>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("Couldn't get db connection from pool");

    let customer = contract::customer::Customer {
        id: user_uid.to_string(),
        document: form.document.clone(),
        name: form.name.clone(),
        second_name: form.second_name.clone(),
        person_type: form.person_type.clone(),
        device_id: form.device_id.clone(),
    };

    let user = web::block(move || dao::customer::update(customer, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/rest/customer/{id}")]
async fn delete_customer(
    pool: web::Data<DbPool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {

    let conn = pool.get().expect("Couldn't get db connection from pool");

    web::block(move || dao::customer::delete(user_uid.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::NoContent().finish())
}
