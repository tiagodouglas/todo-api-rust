use actix_web::{post, web, HttpResponse, Responder};
use application::todo::create::{create_todo, CreateTodoError, CreateTodoRequest};
use shared::responses::ErrorResponse;

#[post("/todo")]
async fn create_todo_handler(todo_request: web::Json<CreateTodoRequest>) -> impl Responder {
    match create_todo(todo_request.into_inner()) {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => match err {
            CreateTodoError { message, status } if status == 500 => HttpResponse::InternalServerError().json(ErrorResponse { message }),
            CreateTodoError { message, status: _ } => HttpResponse::BadRequest().json(ErrorResponse { message }),
        },
    }
}
