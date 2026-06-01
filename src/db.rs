use crate::{models::Recipe, schema::recipes};
use diesel::{Connection, PgConnection, RunQueryDsl, result::Error};
use dotenvy::dotenv;
use std::env;

/// Establish a database connection.
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Stupid stuff test
pub fn __test(conn: &mut PgConnection) -> Result<Vec<Recipe>, Error> {
    recipes::table.load::<Recipe>(conn)
}
