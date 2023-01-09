use diesel::prelude::*;
#[allow(unused_imports)]
use dotenv::dotenv;
use diesel::PgConnection;

#[derive(Clone)]
pub struct Database1 {
}

#[derive(Clone)]
pub struct Database {
}

impl Database {
    pub fn connect() -> PgConnection {
        dotenv::dotenv().ok();
        PgConnection::establish(
            &std::env::var("DATABASE_URL").expect("Error loading the database url"),
        )
        .expect("error loading database")
    }
}