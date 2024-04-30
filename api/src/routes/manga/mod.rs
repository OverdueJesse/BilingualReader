mod helpers;
mod images;
mod structs;
use crate::manga::structs::Manga;
use helpers::{list_dir};
use images::get_image;
use rocket::serde::json::Json;
use crate::routes::manga::helpers::{get_all_manga, get_nth_volume, get_single_manga};
use crate::common::{file_types::FileTypes, lang::Lang};
use crate::routes::manga::structs::{Volume, VolumeList};

#[get("/")]
pub fn view_manga() -> Json<Vec<Manga>> {
    Json(get_all_manga())
}

#[get("/<title>")]
pub fn get_manga(title: &str) -> Json<VolumeList> {
    Json(get_single_manga(title.to_string()))
}

#[get("/<title>/<lang>/<volume>/<page>")]
pub fn get_page(lang: Lang, title: &str, volume: usize, page: usize) -> Json<Vec<u8>> {
    let path = match get_nth_volume(lang, title, volume) {
        Some(s) => s,
        None => return Json(vec![]),
    };
    println!("path: {}", path);

    Json(get_image(path.as_str(), page))
}
