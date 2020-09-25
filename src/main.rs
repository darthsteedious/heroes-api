pub mod domain;
pub mod infrastructure;


use actix_web::{web, App, HttpServer};
use crate::infrastructure::config::Config;
use crate::infrastructure::handlers::register_handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(Config::from_json("appsettings.json")
                .expect("error loading appsettings.json")
                .pg
                .create_pool(tokio_postgres::NoTls)
                .expect("error getting pg pool"))
            .service(register_handlers(web::scope("/api/v1")))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
