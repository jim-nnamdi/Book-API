#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod models;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DB_URL must be set");

    SqliteConnection::establish(&database_url).expect(&format!{"error connecting to the database {}", database_url})
}