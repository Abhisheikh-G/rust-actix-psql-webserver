mod config;
mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder};
use dotenv::dotenv;

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Ok".to_string()
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Starting server at http://127.0.0.1:8080/");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


