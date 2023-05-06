use std::env;

use actix_web::dev::ServiceRequest;
use actix_web::middleware::Logger;
use actix_web::{App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
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
        let auth = HttpAuthentication::bearer(validator);

        App::new()
            .wrap(Logger::new("request: %r status: %s"))
            .service(authenticate_handler)
            .wrap(auth)
            .service(create_user_handler)
            .service(create_todo_handler)
    })
    .bind(("127.0.0.1", port.parse().unwrap()))?
    .run()
    .await
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match application::auth::jwt::validate_token(credentials.token()) {
        Ok(_) => Ok(req),
        Err(_) => Err(AuthenticationError::from(config).into()).map_err(|err| (err, req)),
    }
}
