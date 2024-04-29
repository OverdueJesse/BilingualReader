use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Manga {
    pub title: String,
    pub lang: String,
}

#[derive(Serialize, Deserialize)]
pub struct MangaThumbnail {
    pub title: String,
    pub lang: String,
    pub img: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct MangaVolume {
    pub title: String,
}
