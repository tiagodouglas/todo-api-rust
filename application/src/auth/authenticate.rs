use serde::{Deserialize, Serialize};
use super::jwt::generate_token;

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

pub fn authenticate_user(auth_request: AuthRequest) -> AuthResponse {
    let sub = auth_request.email;

    let token =  generate_token(&sub).unwrap();

    return AuthResponse {
        token: token,
    };
}
