use actix_web::{web};
use super::services::{index, get_entries, create_entry, update_entry, delete_entry};



pub fn init_todolist(cfg: &mut web::ServiceConfig) {
    cfg
        .service(index)
        .service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);



}