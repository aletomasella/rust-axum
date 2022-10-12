
use actix_web::{ App, HttpServer};
use rust_actix_api::endpoints::endpoints::init;


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}