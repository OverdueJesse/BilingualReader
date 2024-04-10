// use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Lang {
    EN,
    JP,
}

// impl fmt::Display for Lang {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Lang::EN => write!(f, "en"),
//             Lang::JP => write!(f, "jp"),
//         }
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct Manga {
    pub title: String,
    pub description: String,
    pub lang: String, // pub lang: Lang,
}

#[derive(Serialize, Deserialize)]
pub struct MangaThumbnail {
    pub title: String,
    pub lang: String
}

#[derive(Serialize, Deserialize)]
pub struct ApiTest {
    pub title: String,
    pub description: String,
}
