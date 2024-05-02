mod helpers;
mod images;
mod structs;
use crate::manga::structs::Manga;
use helpers::{list_dir};
use images::get_image;
use rocket::serde::json::Json;
use crate::routes::manga::helpers::{get_all_manga, get_nth_volume, get_single_manga, get_volume_metadata};
use crate::common::{file_types::FileTypes, lang::Lang};
use crate::routes::manga::structs::{Page, Volume, VolumeList, VolumeMetadata};

#[get("/")]
pub fn view_manga() -> Json<Vec<Manga>> {
    Json(get_all_manga())
}

#[get("/<title>")]
pub fn get_manga(title: &str) -> Json<VolumeList> {
    Json(get_single_manga(title.to_string()))
}

#[get("/<title>/<lang>/<volume>/<page>")]
pub fn get_page(lang: Lang, title: &str, volume: usize, page: usize) -> Json<Page> {
    let blank_page = Page{
        img: vec![],
        metadata: VolumeMetadata{
            page_count: -1,
        }
    };

    let path = match get_nth_volume(lang, title, volume) {
        Some(s) => s,
        None => return Json(blank_page),
    };

    let img = get_image(path.as_str(), page);
    let metadata = get_volume_metadata(path);

    Json(Page {
        img,
        metadata
    })
}
