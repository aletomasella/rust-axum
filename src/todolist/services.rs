use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
// use crate::{AppState, TodoListEntry};
use super::models::{CreateEntryData, UpdateEntryData, DeleteEntryData, GetEntryData};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Returning a list of all entries")
}


