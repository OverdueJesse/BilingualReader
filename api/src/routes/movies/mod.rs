use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

// const MOVIE_ROUTE: &str = "B:\\漫画";

#[derive(Serialize, Deserialize)]
pub struct ApiTest {
  pub title: String,
  pub description: String
}

#[get("/")]
pub fn index() -> Json<ApiTest> {
  return Json(ApiTest {
    title: String::from("Hello"),
    description: String::from("MOvies")
  });
}
