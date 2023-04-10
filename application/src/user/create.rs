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
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
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
    match user_request.validate() {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }

    let email_auth = user_request.email.unwrap();
    let password_auth = user_request.password.unwrap();

    let result = std::panic::catch_unwind(|| {
        let user_exists = match users::table
            .select(users::id)
            .filter(email.eq(&email_auth))
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

        let hashed_password = match hash(password_auth, DEFAULT_COST) {
            Ok(h) => h,
            Err(_) => panic!("Error generating hash"),
        };

        let user = NewUser {
            email: email_auth,
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

impl CreateUserRequest {
    pub fn validate(&self) -> Result<(), CreateUserError> {
        if self.email.is_none() {
            return Err(CreateUserError {
                message: "Email is required".to_owned(),
                status: 400,
            });
        } else if self.password.is_none() {
            return Err(CreateUserError {
                message: "Password is required".to_owned(),
                status: 400,
            });
        }
        Ok(())
    }
}
