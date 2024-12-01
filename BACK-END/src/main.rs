use actix_web::{web, App, HttpServer};
use colored::*;
mod data;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(std::sync::Mutex::new(data::read_data()));

    println!(
        "{}",
        "🚀 Server is running at http://127.0.0.1:8080"
            .green()
            .bold()
    );
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(handlers::config_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
