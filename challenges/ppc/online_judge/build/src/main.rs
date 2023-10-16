use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use oj::api::{get_jobs, hello, post_jobs};
use oj::Cli;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
struct ErrorMessage {
    code: i32,
    reason: String,
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let cli = Cli::parse();
    let config = cli.init_config()?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .wrap(Logger::default())
            .service(hello)
            .service(get_jobs)
            .service(post_jobs)
    })
    .bind(("0.0.0.0", 12345))?
    .run()
    .await
}
