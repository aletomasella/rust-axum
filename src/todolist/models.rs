use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateEntryData {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateEntryData {
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct DeleteEntryData {
    pub id: u32,
}

#[derive(Deserialize)]
pub struct GetEntryData {
    pub id: u32,
}