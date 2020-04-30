use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Customer {
    pub id: i32,
    pub document: String,
    pub name: String,
    #[serde(alias="secondName")]
    pub second_name: String,
    #[serde(alias="personType")]
    pub person_type: String,
    #[serde(alias="deviceId")]
    pub device_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Info {
    pub id: String
}

#[derive(Deserialize)]
pub struct QueryCustomer {
    #[serde(default)]
    pub document: String,
    #[serde(default)]
    pub name: String,
    #[serde(alias="secondName", default)]
    pub second_name: String,
    #[serde(alias="personType", default)]
    pub person_type: String,
    #[serde(alias="updatedFrom", default)]
    pub updated_from: u128,
    #[serde(alias="updatedTo", default)]
    pub updated_to: u128,
}
