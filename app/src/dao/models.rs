#[derive(Queryable)]
pub struct Customer {
    pub id: i32,
    pub document: String,
    pub name: String,
    pub second_name: String,
    pub person_type: String,
    pub device_id: String,
}
