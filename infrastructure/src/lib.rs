use diesel::{Connection, PgConnection};
use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))

    // let connection = match AsyncPgConnection::establish(&database_url).await {
    //     Ok(con) => con,
    //     Err(_) => panic!("Error connecting to the database")
    // };

    // return connection;
}
