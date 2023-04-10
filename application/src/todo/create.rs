use chrono::Utc;
use diesel::prelude::*;

use domain::{
    schema::{users::{self, email}, todos}, todo::{Todo, NewTodo},
};
use infrastructure::establish_connection;
use serde::{Deserialize, Serialize};



#[derive(Deserialize)]
pub struct CreateTodoRequest {
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "completed")]
    pub completed: Option<bool>,
}

#[derive(Serialize)]
pub struct CreateTodoResponse {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize)]
pub struct CreateTodoError {
    pub message: String,
    pub status: i32,
}

pub fn create_todo(todo_request: CreateTodoRequest) -> Result<CreateTodoResponse, CreateTodoError> {
    match todo_request.validate() {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }

    let description = todo_request.description.unwrap();
    let completed = todo_request.completed.unwrap();
    let email_teste = "tiago9@teste.com"; // TODO: get from token

    let result = std::panic::catch_unwind(|| {
        let user_id = match users::table
            .select(users::id)
            .filter(email.eq(&email_teste))
            .first::<i32>(&mut establish_connection())
        {
            Ok(id) => id,
            Err(diesel::NotFound) => 0,
            Err(err) => panic!("Database error: {}", err),
        };

        if user_id == 0 {
            return Err(CreateTodoError {
                message: "Could not find the user".to_owned(),
                status: 400,
            });
        }

        let todo = NewTodo {
            description: description,
            completed: completed,
            userid: user_id,
            datecreated: Utc::now()
        };

        match diesel::insert_into(todos::table)
            .values(&todo)
            .get_result::<Todo>(&mut establish_connection())
        {
            Ok(t) => {
                let response = CreateTodoResponse {
                    id: t.id,
                    description: t.description,
                    completed: t.completed
                 
                };

                return Ok(response);
            }
            Err(err) => match err {
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        }
    });

    match result {
        Ok(res) => res,
        Err(_) => Err(CreateTodoError {
            message: "Internal Server Error".to_owned(),
            status: 500,
        }
        .into()),
    }
}

impl CreateTodoRequest {
    pub fn validate(&self) -> Result<(), CreateTodoError> {
        if self.description.is_none() {
            return Err(CreateTodoError {
                message: "Description is required".to_owned(),
                status: 400,
            });
        } else if self.completed.is_none() {
            return Err(CreateTodoError {
                message: "Completed is required".to_owned(),
                status: 400,
            });
        }
        Ok(())
    }
}
