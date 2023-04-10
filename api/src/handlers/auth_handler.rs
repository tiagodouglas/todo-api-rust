use actix_web::{post, web, HttpResponse, Responder};
use application::auth::authenticate::{authenticate_user, AuthError, AuthRequest};
use shared::responses::ErrorResponse;

#[post("/auth")]
async fn authenticate_handler(auth_request: web::Json<AuthRequest>) -> impl Responder {
    match authenticate_user(auth_request.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => match err {
            AuthError { message, status } if status == 500 => {
                HttpResponse::InternalServerError().json(ErrorResponse { message })
            }
            AuthError { message, status: _ } => {
                HttpResponse::BadRequest().json(ErrorResponse { message })
            }
        },
    }
}
