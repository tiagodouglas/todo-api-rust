use actix_web::{post, web, Responder, HttpResponse};
use application::user::create::{CreateUserRequest, create_user, CreateUserError};
use shared::responses::ErrorResponse;

#[post("/user")]
async fn create_user_handler(user_request: web::Json<CreateUserRequest>) -> impl Responder {
    match create_user(user_request.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => match err {
            CreateUserError { message, status } => {
                if status == 500 {
                    HttpResponse::InternalServerError().json(ErrorResponse {
                        message,
                    })
                } else {
                    HttpResponse::BadRequest().json(ErrorResponse { message })
                }
            }
        },
    }
}
