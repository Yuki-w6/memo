use actix_web::{web, App, HttpServer};

async fn index() -> String {
    "Hello from Docker!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    })
    .bind("memo_app:8080")?
    .run()
    .await
}
