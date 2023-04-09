use super::jwt::generate_token;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use domain::{
    schema::users::{self, email},
    user::User,
};
use infrastructure::establish_connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    exp: usize
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AuthError {
    pub message: String,
    pub status: i32,
}

pub fn authenticate_user(auth_request: AuthRequest) -> Result<AuthResponse, AuthError> {
    let result = std::panic::catch_unwind(|| {
        let user = match users::table
            .filter(users::email.eq(email))
            .first::<User>(&mut establish_connection())
        {
            Ok(u) => Some(u),
            Err(diesel::NotFound) => None,
            Err(err) => {
                return Err(AuthError {
                    message: format!("Database error: {}", err),
                    status: 500,
                })
            }
        };

        if user.is_none() {
            return Err(AuthError {
                message: "User not found".to_owned(),
                status: 400,
            });
        }

        let user = user.unwrap();
        let hashed_password = &user.hash;

        let password_matches = verify(auth_request.password, hashed_password).unwrap();

        if !password_matches {
            return Err(AuthError {
                message: "Email or password is invalid".to_owned(),
                status: 400,
            });
        }

        let sub = auth_request.email;

        let token = generate_token(&sub).unwrap();

        return Ok(AuthResponse { token: token.token, exp: token.exp });
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
