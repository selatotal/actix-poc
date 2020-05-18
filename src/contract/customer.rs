use serde::{Deserialize, Serialize};
use crate::dao::schema::customers;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Customer{
    pub id: String,
    pub document: String,
    pub name: String,
    #[serde(rename="secondName")]
    pub second_name: Option<String>,
    #[serde(rename="personType")]
    pub person_type: String,
    #[serde(rename="deviceId")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCustomer{
    pub document: String,
    pub name: String,
    #[serde(rename="secondName")]
    pub second_name: Option<String>,
    #[serde(rename="personType")]
    pub person_type: String,
    #[serde(rename="deviceId")]
    pub device_id: Option<String>,
}
