use std::fs;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// const MANGA_ROUTE: &str = "B:\\漫画";

#[derive(Serialize, Deserialize)]
pub struct ApiTest {
    pub title: String,
    pub description: String,
}

// pub struct Manga {
//   pub title: String,
//   pub description: String
// }

#[get("/")]
pub fn index() -> Json<Vec<ApiTest>> {
    let paths = fs::read_dir("B:\\漫画\\日本語").unwrap();
    let mut manga_list: Vec<String> = vec![];

    for path in paths {
        let pathname = String::from(path.as_ref().unwrap().file_name().to_str().unwrap());
        if !pathname.starts_with('.') {
            manga_list.push(pathname);
        }
    }

    let mut res: Vec<ApiTest> = vec![];
    for manga in manga_list {
        res.push(ApiTest {
            title: manga,
            description: String::from("Manga"),
        });
    }

    return Json(res);
}

#[get("/test")]
pub fn test() -> Json<ApiTest> {
    return Json(ApiTest {
        title: String::from("Hello"),
        description: String::from("Test"),
    });
}
