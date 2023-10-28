#[macro_use]
extern crate diesel;
extern crate dotenv;
use actix_web::{App, HttpServer};

mod schema;
mod database;
mod state;
mod to_do;
mod json_serialization;
mod views;
mod processes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .configure(views::views_factory);
        app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
