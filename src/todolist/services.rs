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
    let mut id = 0;
    if entries.len() > 0 {
        id = entries[entries.len() - 1].id + 1;
    }
    let title = entry.title.clone();
    let description = entry.description.clone();
    let new_entry = TodoListEntry::new(id, title, description);
    entries.push(new_entry.clone());
    HttpResponse::Ok().json(entries.to_vec())
}

#[put("/entries/{id}")]
async fn update_entry(data : web::Data<AppState>, entry : web::Json<UpdateEntryData>, id : web::Path<u32>) -> impl Responder {
    let mut entries = data.todolist_entries.lock().unwrap();
    let title = entry.title.clone();
    let description = entry.description.clone();

    for entry in entries.iter_mut() {
        if entry.id == *id {
            entry.title = title.clone();
            entry.description = description.clone();
        }
    }

    HttpResponse::Ok().json(entries.to_vec())
}


#[delete("/entries/{id}")]
async fn delete_entry(data : web::Data<AppState>, id : web::Path<u32>) -> impl Responder {
    let mut entries = data.todolist_entries.lock().unwrap();
    let mut idx = 0;
    for entry in entries.iter() {
        if entry.id == *id {
            break;
        }
        idx += 1;
    }
    entries.remove(idx);
    HttpResponse::Ok().json(entries.to_vec())
}


