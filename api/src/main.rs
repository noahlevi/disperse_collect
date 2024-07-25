mod api;
mod contract;

use actix_web::{App, HttpServer};
use api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(disperse).service(collect))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
