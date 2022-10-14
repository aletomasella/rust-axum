
use actix_web::{ App, HttpServer, web};
use rust_actix_api::endpoints::endpoints::init;
use rust_actix_api::todolist::controllers::init_todolist;
use rust_actix_api::config::{read_config};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;


pub struct AppState {
    todolist_entries: Mutex<Vec<TodoListEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoListEntry {
    id: u32,
    title: String,
    description: String,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
let config = read_config();

let app_data = web::Data::new(AppState {
    todolist_entries: Mutex::new(vec![]),
});

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::scope("").configure(init))
            .service(web::scope("/todolist").configure(init_todolist))
    })
    .bind((config.host, config.port))? // <- bind address
    .run()
    .await
}