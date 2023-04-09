use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Selectable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hash: String,
    pub datecreated: DateTime<Utc>,
    pub dateupdated: Option<DateTime<Utc>>,
}

#[derive(Insertable, Deserialize)]
#[serde()]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub hash: String,
    pub datecreated: DateTime<Utc>,
    // pub dateupdated: Option<DateTime<Utc>>,
}
