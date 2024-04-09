#[macro_use]
extern crate rocket;

mod routes;
use routes::{anime, manga, movies};

#[launch]
fn rocket() -> _ {
    let manga_routes = routes![manga::index, manga::view_manga, manga::get_manga, manga::test,];
    let anime_routes = routes![anime::index];
    let movie_routes = routes![movies::index];

    rocket::build()
        .mount("/manga", manga_routes)
        .mount("/anime", anime_routes)
        .mount("/movies", movie_routes)
}
