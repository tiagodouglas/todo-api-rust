use crate::schema::todos;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Selectable)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub completed: bool,
    pub userid: i32,
    pub datecreated: DateTime<Utc>,
    pub dateupdated: Option<DateTime<Utc>>,
}

#[derive(Insertable, Deserialize)]
#[serde()]
#[diesel(table_name = todos)]
pub struct NewTodo {
    pub description: String,
    pub completed: bool,
    pub userid: i32,
    pub datecreated: DateTime<Utc>,
}
