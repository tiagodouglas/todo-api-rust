use actix_web::{App, HttpServer};
use api::handlers::auth_handler::authenticate_handler;
use api::handlers::user_handler::create_user_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(authenticate_handler)
            .service(create_user_handler)
    })
    .bind(("127.0.0.1", 5500))?
    .run()
    .await
}
