use diesel;
use self::models::*;
use self::schema::*;
use self::diesel::prelude::*;

pub fn get_all() {c
    let connection = establish_connection();
    let results = CUSTOMER.load::<Customer>(&connection);
    for customer in results {
        println!("{:?}", customer);
    }
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
   let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}