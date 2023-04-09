use actix_web::{post, web, Responder, HttpResponse};
use application::auth::authenticate::{authenticate_user, AuthRequest, AuthError};
use shared::responses::ErrorResponse;

#[post("/auth")]
async fn authenticate_handler(auth_request: web::Json<AuthRequest>) -> impl Responder {
    match authenticate_user(auth_request.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => match err {
            AuthError { message, status } => {
                if status == 500 {
                    HttpResponse::InternalServerError().json(ErrorResponse { message })
                } else {
                    HttpResponse::BadRequest().json(ErrorResponse { message })
                }
            }
        },
    }
}
