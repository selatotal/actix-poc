use actix_web::{web, HttpResponse, Responder, Result};
use log::*;
use crate::contract::v1::payload::*;

pub async fn save(input: web::Json<Customer>) -> Result<HttpResponse> {
    debug!("Create new customer document[{}] name[{}] second_name[{}]", input.document, input.name, input.second_name);
    Ok(HttpResponse::Ok().json(Customer{
        id: 1,
        document: String::from("08491597000126"),
        name: String::from("ORSEGUPS"),
        second_name: String::from("ORSEGUPS MONITORAMENTO ELETRONICO LTDA"),
        person_type: String::from("J"),
        device_id: String::from("892962556416042759408152718178207591774500325695360457545052"),
    }))
}

pub async fn list() -> impl Responder {
    HttpResponse::Ok().body("Get all!")
}

pub async fn query(query: web::Query<QueryCustomer>) -> impl Responder {
    HttpResponse::Ok().body(format!("Query all! : document[{}] name[{}], secondName[{}], personType[{}], {}, {}",
        query.document,
        query.name,
        query.second_name,
        query.person_type,
        query.updated_from,
        query.updated_to)
    )
}

pub async fn get_by_id(path: web::Path<Info>) -> Result<HttpResponse> {
    debug!("Get By Id! {}", path.id);
    Ok(HttpResponse::Ok().json(Customer{
        id: 1,
        document: String::from("08491597000126"),
        name: String::from("ORSEGUPS"),
        second_name: String::from("ORSEGUPS MONITORAMENTO ELETRONICO LTDA"),
        person_type: String::from("J"),
        device_id: String::from("892962556416042759408152718178207591774500325695360457545052"),
    }))
}

pub async fn update(path: web::Path<Info>, input: web::Json<Customer>) -> Result<HttpResponse> {
    debug!("Update of id {}! document[{}] name[{}] second_name[{}]", path.id, input.document, input.name, input.second_name);
    Ok(HttpResponse::Ok().json(Customer{
        id: 1,
        document: String::from("08491597000126"),
        name: String::from("ORSEGUPS"),
        second_name: String::from("ORSEGUPS MONITORAMENTO ELETRONICO LTDA"),
        person_type: String::from("J"),
        device_id: String::from("892962556416042759408152718178207591774500325695360457545052"),
    }))
}

pub async fn delete(path: web::Path<Info>) -> impl Responder {
    debug!("Delete {}!", path.id);
    HttpResponse::NoContent()
}
