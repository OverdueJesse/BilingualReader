use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Lang {
    EN,
    JP,
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lang::EN => write!(f, "en"),
            Lang::JP => write!(f, "jp"),
        }
    }
}

pub enum FileTypes {
    CBZ,
    ZIP,
    FOLDER
}

impl FileTypes {
    pub fn to_bool(&self, s: &str) -> bool {
        match self {
            FileTypes::CBZ => s.contains(".cbz"),
            FileTypes::ZIP => s.contains(".zip"),
            FileTypes::FOLDER => !s.contains("."),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            FileTypes::CBZ => ".cbz".to_string(),
            FileTypes::ZIP => ".zip".to_string(),
            FileTypes::FOLDER => "Folder".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Manga {
    pub title: String,
    pub lang: String,
}

#[derive(Serialize, Deserialize)]
pub struct MangaThumbnail {
    pub title: String,
    pub lang: String
}
