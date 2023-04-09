use actix_web::{http, post, web, Responder};
use application::auth::authenticate::{authenticate_user, AuthRequest};

#[post("/auth")]
async fn authenticate_handler(auth_request: web::Json<AuthRequest>) -> impl Responder {
    let response = authenticate_user(auth_request.into_inner());

    (web::Json(response), http::StatusCode::OK)
}
