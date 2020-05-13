table! {
    customers (id) {
        id -> Varchar,
        document -> Varchar,
        name -> Varchar,
        second_name -> Nullable<Varchar>,
        person_type -> Char,
        device_id -> Nullable<Varchar>,
    }
}
