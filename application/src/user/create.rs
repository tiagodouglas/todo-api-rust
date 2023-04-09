use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use diesel::prelude::*;
use domain::{
    schema::users::{self, email},
    user::{NewUser, User},
};
use infrastructure::establish_connection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: i32,
    pub email: String,
}

#[derive(Serialize)]
pub struct CreateUserError {
    pub message: String,
    pub status: i32,
}

pub fn create_user(user_request: CreateUserRequest) -> Result<CreateUserResponse, CreateUserError> {
    let result = std::panic::catch_unwind(|| {
        let user_exists = match users::table
            .select(users::id)
            .filter(email.eq(&user_request.email))
            .first::<i32>(&mut establish_connection())
        {
            Ok(_) => true,
            Err(diesel::NotFound) => false,
            Err(err) => panic!("Database error: {}", err),
        };

        if user_exists {
            return Err(CreateUserError {
                message: "User already exists".to_owned(),
                status: 400,
            });
        }

        let hashed_password = match hash(user_request.password, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => panic!("Error generating hash"),
        };

        let user = NewUser {
            email: user_request.email,
            hash: hashed_password,
            datecreated: Utc::now(),
        };

        match diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(&mut establish_connection())
        {
            Ok(u) => {
                let response = CreateUserResponse {
                    id: u.id,
                    email: u.email,
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
        Err(_) => Err(CreateUserError {
            message: "Internal Server Error".to_owned(),
            status: 500,
        }
        .into()),
    }
}
