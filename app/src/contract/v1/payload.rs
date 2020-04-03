use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Customer {
    pub id: String,
    pub document: String,
    pub name: String,
    #[serde(alias="secondName")]
    pub second_name: String,
    #[serde(alias="personType")]
    pub person_type: String,
    #[serde(alias="deviceId")]
    pub device_id: String,
    pub tags: Vec<CustomerTag>
}

#[derive(Serialize,Deserialize)]
pub struct CustomerTag {
    pub id: String,
    pub name: String,
    #[serde(alias="type")]
    pub tag_type: String,
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
    #[serde(alias="tagIds", default)]
    pub tag_ids: Vec<String>,
    #[serde(alias="updatedFrom", default)]
    pub updated_from: u128,
    #[serde(alias="updatedTo", default)]
    pub updated_to: u128,
}

#[derive(Deserialize)]
pub struct QueryTags {
    #[serde(default)]
    pub name: String,
    #[serde(alias="type")]
    pub query_type: String,
}