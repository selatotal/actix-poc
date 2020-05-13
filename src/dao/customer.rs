use diesel::prelude::*;
use uuid::Uuid;

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