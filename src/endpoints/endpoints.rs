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

#[get("/hello/{name}/{lastname}")]
async fn hello_with_lastname(name : web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {} {}!", name.0, name.1))
}

#[get("/hello/{name}/{lastname}/{age}")]
async fn hello_with_lastname_and_age(name : web::Path<(String, String, u8)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {} {}! You are {} years old.", name.0, name.1, name.2))
}



pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(version)
        .service(welcome)
        .service(hello)
        .service(hello_with_lastname)
        .service(hello_with_lastname_and_age);
}
