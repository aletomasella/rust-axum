
use actix_web::{ App, HttpServer, web};
use rust_actix_api::endpoints::endpoints::init;
use rust_actix_api::config::{read_config};


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
let config = read_config();

    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(init))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}