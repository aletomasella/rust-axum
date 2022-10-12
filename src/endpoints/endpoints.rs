use actix_web::{get, web, Responder, HttpResponse};


#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Welcome to my website!")
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body("0.1.0")
}

#[get("/hello/{name}")]
async fn hello(name : web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name))
}


pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(version)
        .service(welcome);
}
