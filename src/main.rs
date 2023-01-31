use actix_web::{App, HttpServer, middleware};

use torch_service::app_config::config_app;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "full");
    HttpServer::new(|| {
        App::new()
            .configure(config_app)
            .wrap(middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}