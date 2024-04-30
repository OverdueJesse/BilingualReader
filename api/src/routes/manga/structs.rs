use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Manga {
    pub title: LangOptions,
    pub volume_count: i32,
    pub thumbnail: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub title: LangOptions,
    pub volume_count: i32,
    pub thumbnail: String,
}

#[derive(Serialize, Deserialize)]
pub struct Volume {
    pub title: String,
    // pub volume_number: i32,
    // pub page_count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct VolumeList {
    pub en: Vec<Volume>,
    pub jp: Vec<Volume>,
}

#[derive(Serialize, Deserialize)]
pub struct LangOptions {
    pub en: String,
    pub jp: String,
}
