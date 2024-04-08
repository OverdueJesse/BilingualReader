use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// const ANIME_ROUTE: &str = "B:\\アニメ";

#[derive(Serialize, Deserialize)]
pub struct Anime {
  pub title: String,
  pub thumbnail: String
}

#[get("/")]
pub fn index() -> Json<Anime> {
  return Json(Anime {
    title: String::from("Hello"),
    thumbnail: String::from("Anime")
  });
}
