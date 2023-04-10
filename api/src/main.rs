use std::env;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use api::handlers::auth_handler::authenticate_handler;
use api::handlers::todo_handler::create_todo_handler;
use api::handlers::user_handler::create_user_handler;
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv().ok();

    let port = env::var("PORT").expect("PORT must be set.");

    println!("Server started successfully on port {port}");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("request: %r status: %s"))
            .service(authenticate_handler)
            .service(create_user_handler)
            .service(create_todo_handler)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}
