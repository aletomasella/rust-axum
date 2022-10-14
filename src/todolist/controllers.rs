use actix_web::{web};
use super::services::{index};



pub fn init_todolist(cfg: &mut web::ServiceConfig) {
    cfg
        .service(index);

}