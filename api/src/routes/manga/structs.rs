use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Manga {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiTest {
    pub title: String,
    pub description: String,
}