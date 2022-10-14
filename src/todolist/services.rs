use actix_web::{get, post, put, delete, web, Responder, HttpResponse, App};
use crate::data::{TodoListEntry, AppState};
use super::models::{CreateEntryData, UpdateEntryData, DeleteEntryData, GetEntryData};

#[get("")]
async fn index() -> String {
    String::from("Welcome to my website!")
}

#[get("/entries")]
async fn get_entries(data : web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todolist_entries.lock().unwrap().to_vec())
}


#[post("/entries")]
async fn create_entry(data : web::Data<AppState>, entry : web::Json<CreateEntryData>) -> impl Responder {
    let mut entries = data.todolist_entries.lock().unwrap();
    let id = entries.len() as u32;
    let title = entry.title.clone();
    let description = entry.description.clone();
    let new_entry = TodoListEntry::new(id, title, description);
    entries.push(new_entry.clone());
    HttpResponse::Ok().json(entries.to_vec())
}


