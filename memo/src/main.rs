use actix_web::{get, web, App, HttpServer, Responder};
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn hello(db_pool: web::Data<PgPool>) -> impl Responder {
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(&**db_pool)
        .await
        .unwrap();
    
    format!("Database says: {}", row.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create a connection pool
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create DB pool.");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .service(hello)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
