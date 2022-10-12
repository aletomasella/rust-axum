use actix_web::{get, web, App, HttpServer, Responder};

// struct WelcomeBody {
//     message: String,
// }
 
#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/")]
async fn welcome() -> impl Responder {
    format!("Welcome to my website!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}