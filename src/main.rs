mod handlers;
mod models;
mod config;

use actix_web::{web,App, HttpServer, middleware::Logger};
use actix_cors::Cors; // CORS를 사용하기 위해 추가
use actix_web::http::header;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=debug");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await 
        {
            Ok(pool) => {
                println!("Connected successfully to Postgres");
                pool
            }
            Err(e) => {
                println!("Failed to connect to Postgres: {}", e);
                panic!()
            }
        };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::CONTENT_TYPE])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config::router::config)
            .wrap(Logger::default()) // 로깅 미들웨어 추가
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
