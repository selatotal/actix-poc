use diesel::prelude::*;
use uuid::Uuid;
use log::*;
use crate::contract;
use crate::dao::schema;

pub fn find_customer_by_id(
    uid: Uuid,
    conn: &MysqlConnection,
) -> Result<Option<contract::customer::Customer>, diesel::result::Error>{
    use schema::customers::dsl::*;
    let customer = customers
        .filter(id.eq(uid.to_string()))
        .first::<contract::customer::Customer>(conn)
        .optional()?;
    Ok(customer)
}

pub fn insert(
    customer: contract::customer::Customer,
    conn: &MysqlConnection,
) -> Result<contract::customer::Customer, diesel::result::Error>{
    use schema::customers::dsl::*;
    diesel::insert_into(customers)
        .values(&customer)
        .execute(conn)?;
    Ok(customer)
}

pub fn update(
    customer: contract::customer::Customer,
    conn: &MysqlConnection,
) -> Result<contract::customer::Customer, diesel::result::Error>{
    use schema::customers::dsl::*;
    info!("Updating");
    diesel::update(customers)
        .set(&customer)
        .execute(conn)?;
    Ok(customer)
}

pub fn delete(
    uid: Uuid,
    conn: &MysqlConnection,
) -> Result<bool, diesel::result::Error> {
    use schema::customers::dsl::*;
    let num_deleted = diesel::delete(customers.filter(id.eq(uid.to_string())))
        .execute(conn)?;
    Ok(num_deleted>0)
}

pub fn get_all(
    conn: &MysqlConnection,
) -> Result<Vec<contract::customer::Customer>, diesel::result::Error>{
    use schema::customers::dsl::*;
    let result = customers.load::<contract::customer::Customer>(conn)?;
    Ok(result)    
}