use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::sessions;

/// For the JSON body in POST /api/create
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRequest {
    pub hostname: String,
    pub ip_address: Option<String>,
}

/// For the JSON response after creation
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    pub token: String,
    pub full_url: String,
    pub domain: String,
}

/// The "sessions" table record
#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub token: String,
    pub hostname: String,
    pub ip_address: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Used when inserting a row into "sessions"
#[derive(Debug, Insertable)]
#[diesel(table_name = sessions)]
pub struct NewSession<'a> {
    pub token: &'a str,
    pub hostname: &'a str,
    pub ip_address: Option<&'a str>,
}
