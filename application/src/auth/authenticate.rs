use super::jwt::generate_token;
use bcrypt::verify;
use diesel::prelude::*;
use domain::{
    schema::users::{self},
    user::User,
};
use infrastructure::establish_connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    exp: usize,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "password")]
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct AuthError {
    pub message: String,
    pub status: i32,
}

pub fn authenticate_user(auth_request: AuthRequest) -> Result<AuthResponse, AuthError> {
    match auth_request.validate() {
        Ok(_) => {}
        Err(err) => {
            return Err(err);
        }
    }

    let email_auth = auth_request.email.unwrap();
    let password_auth = auth_request.password.unwrap();

    let result = std::panic::catch_unwind(|| {
        let user = match users::table
            .filter(users::email.eq(&email_auth))
            .first::<User>(&mut establish_connection())
        {
            Ok(u) => u,
            Err(diesel::NotFound) => {
                return Err(AuthError {
                    message: "User not found".to_owned(),
                    status: 404,
                })
            }
            Err(err) => panic!("Database error: {}", err),
        };

        let hashed_password = &user.hash;

        let password_matches = verify(password_auth, hashed_password).unwrap();

        if !password_matches {
            return Err(AuthError {
                message: "Email or password is invalid".to_owned(),
                status: 400,
            });
        }

        let sub = email_auth;

        let token = generate_token(&sub).unwrap();

        return Ok(AuthResponse {
            token: token.token,
            exp: token.exp,
        });
    });

    match result {
        Ok(res) => res,
        Err(_) => Err(AuthError {
            message: "Internal Server Error".to_owned(),
            status: 500,
        }
        .into()),
    }
}

impl AuthRequest {
    pub fn validate(&self) -> Result<(), AuthError> {
        if self.email.is_none() {
            return Err(AuthError {
                message: "Email is required".to_owned(),
                status: 400,
            });
        } else if self.password.is_none() {
            return Err(AuthError {
                message: "Password is required".to_owned(),
                status: 400,
            });
        }
        Ok(())
    }
}
