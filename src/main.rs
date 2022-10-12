
use actix_web::{ App, HttpServer, web};
use rust_actix_api::endpoints::endpoints::init;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(init))
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}