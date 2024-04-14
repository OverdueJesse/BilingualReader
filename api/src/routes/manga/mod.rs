mod helpers;
mod images;
mod structs;
use helpers::{get_lang_path, get_single_manga};
use images::get_image;
use rocket::serde::json::Json;
use structs::{FileTypes, Manga, MangaThumbnail};

use self::{helpers::get_volume_path, structs::Lang};

#[get("/")]
pub fn view_manga() -> Json<Vec<MangaThumbnail>> {
    let mut manga_list: Vec<MangaThumbnail> = vec![];
    for l in vec![Lang::EN, Lang::JP] {
        for title in helpers::list_dir(get_lang_path(l.to_string().as_str()), FileTypes::FOLDER) {
            manga_list.push(MangaThumbnail {
                title,
                lang: l.to_string(),
            })
        }
    }

    Json(manga_list)
}

#[get("/<lang>/<title>")]
pub fn get_manga(lang: &str, title: &str) -> Json<Vec<Manga>> {
    let mut manga_list: Vec<Manga> = vec![];
    for manga in get_single_manga(lang, title) {
        manga_list.push(Manga {
            title: manga,
            lang: lang.to_string(),
        })
    }

    Json(manga_list)
}

#[get("/<lang>/<title>/<volume>/<page>")]
pub fn get_page(lang: &str, title: &str, volume: &str, page: i8) -> Json<Vec<u8>> {
    Json(get_image(&get_volume_path(lang, title, volume), page))
}
