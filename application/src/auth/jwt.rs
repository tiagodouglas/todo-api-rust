use actix_web::Result;
use dotenvy::dotenv;
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub exp: usize,
}

pub fn generate_token(sub: &str) -> Result<Token> {
    dotenv().ok();
    let claims = Claims {
        sub: sub.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp() as usize,
    };

    let secret = match env::var("JWT_SECRET") {
        Ok(s) => s,
        Err(_) => panic!("JWT_SECRET must be set"),
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ) {
        Ok(t) => t,
        Err(_) => panic!("Error generating token"),
    };

    Ok(Token {
        token: token,
        exp: claims.exp,
    })
}

pub fn validate_token(token: &str) -> Result<Claims> {
    dotenv().ok();

    let secret = match env::var("JWT_SECRET") {
        Ok(s) => s,
        Err(_) => panic!("JWT_SECRET must be set"),
    };

    let validation = Validation::new(Algorithm::HS256);

    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"),
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"),
            _ => panic!("Erro validating token"),
        },
    };

    Ok(token_data.claims)
}
