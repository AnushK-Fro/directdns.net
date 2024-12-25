pub mod models;
pub mod schema;

use anyhow::Result;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::{NewSession, Session};
use crate::schema::sessions::dsl;

/// Establish and return a single MySQL connection.
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

/// Insert a new row into `sessions`
pub fn insert_session(
    conn: &mut MysqlConnection,
    token: &str,
    hostname: &str,
    ip_address: Option<&str>,
) -> Result<()> {
    use crate::schema::sessions;

    let new_sess = NewSession {
        token,
        hostname,
        ip_address,
    };

    diesel::insert_into(sessions::table)
        .values(&new_sess)
        .execute(conn)?;

    Ok(())
}

/// Lookup a session by token
pub fn get_session_by_token(
    conn: &mut MysqlConnection,
    the_token: &str,
) -> Result<Option<Session>> {
    // SELECT * FROM sessions WHERE token=? LIMIT 1
    let result = dsl::sessions
        .filter(dsl::token.eq(the_token))
        .limit(1)
        .load::<Session>(conn)?;

    Ok(result.into_iter().next())
}
