
use actix_web::{ App, HttpServer, web};
use rust_actix_api::todolist::controllers::init_todolist;
use rust_actix_api::config::{read_config};
use rust_actix_api::data::{AppState};
use std::sync::Mutex;





#[actix_web::main]
async fn main() -> std::io::Result<()> {
let config = read_config();

let app_data = web::Data::new(AppState {
    todolist_entries: Mutex::new(vec![]),
});

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("/todolist").configure(init_todolist))
    })
    .bind((config.host, config.port))? // <- bind address
    .run()
    .await
}