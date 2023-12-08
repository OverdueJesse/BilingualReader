use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiTest {
  pub title: String,
  pub description: String
}

#[get("/")]
pub fn index() -> Json<ApiTest> {
  return Json(ApiTest {
    title: String::from("Hello"),
    description: String::from("Manga")
  });
}

#[get("/test")]
pub fn test() -> Json<ApiTest> {
  return Json(ApiTest {
    title: String::from("Hello"),
    description: String::from("Test")
  });
}