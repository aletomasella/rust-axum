use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateEntryData {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub id: u32,
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize)]
pub struct DeleteEntryData {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct GetEntryData {
    pub id: u32,
}