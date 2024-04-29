mod helpers;
mod images;
mod structs;
use helpers::{get_lang_path, get_manga_path, get_single_manga, list_dir};
use images::get_image;
use rocket::serde::json::Json;
use structs::{MangaThumbnail};
use crate::routes::manga::helpers::create_manga_thumbnail;
use self::{helpers::get_volume_path};
use crate::common::{file_types::FileTypes, lang::Lang};
use crate::routes::manga::structs::MangaVolume;

#[get("/")]
pub fn view_manga() -> Json<Vec<MangaThumbnail>> {
    let mut manga_list: Vec<MangaThumbnail> = vec![];
    for l in vec![Lang::EN, Lang::JP] {
        let path = match get_lang_path(l) {
            Some(s) => s,
            None => return Json(vec![]),
        };

        for title in list_dir(path, FileTypes::FOLDER) {
            manga_list.push(create_manga_thumbnail(title, l));
        }
    }

    Json(manga_list)
}

#[get("/<lang>/<title>")]
pub fn get_manga(lang: Lang, title: &str) -> Json<Vec<MangaVolume>> {
    let mut volume_list:Vec<MangaVolume> = vec![];

    let volumes = match get_single_manga(lang, title) {
        Some(v) => v,
        None => return Json(vec![]),
    };

    for volume in volumes {
        volume_list.push(MangaVolume {
            title: volume,
        })
    }

    Json(volume_list)
}

#[get("/<lang>/<title>/<volume>/<page>")]
pub fn get_page(lang: Lang, title: &str, volume: &str, page: usize) -> Json<Vec<u8>> {
    match get_volume_path(lang, title, volume) {
        Some(s) => Json(get_image(s.as_str(), page)),
        None => return Json(vec![]),
    }
}
