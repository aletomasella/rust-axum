use actix_web::{get, web, Responder, HttpResponse};


#[get("/")]
async fn index() ->  String {
    String::from("Welcome to my website!")
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body("0.1.0")
}



pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(version)
        .service(index);

}


