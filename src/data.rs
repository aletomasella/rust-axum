use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct AppState {
    pub todolist_entries: Mutex<Vec<TodoListEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoListEntry {
    id: u32,
    title: String,
    description: String,
}

impl TodoListEntry {
    pub fn new(id: u32, title: String, description: String) -> TodoListEntry {
        TodoListEntry {
            id,
            title,
            description,
        }
    }
}