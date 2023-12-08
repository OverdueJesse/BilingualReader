#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

mod routes;
use routes::manga::ApiTest;

#[get("/")]
fn index() -> Json<ApiTest> {
    return Json(ApiTest {
        title: String::from("Hello"),
        description: String::from("World")
      });
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/manga", routes![
            routes::manga::index,
            routes::manga::test
        ])
}
