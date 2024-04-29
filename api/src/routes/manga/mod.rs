mod helpers;
mod images;
mod structs;
use helpers::{get_lang_path, get_manga_path, get_single_manga, list_dir};
use images::get_image;
use rocket::serde::json::Json;
use structs::{FileTypes, Manga, MangaThumbnail};

use self::{helpers::get_volume_path, structs::Lang};

#[get("/")]
pub fn view_manga() -> Json<Vec<MangaThumbnail>> {
    let mut manga_list: Vec<MangaThumbnail> = vec![];
    for l in vec![Lang::EN, Lang::JP] {
        let path = get_lang_path(l.to_string().as_str());

        for title in list_dir(path, FileTypes::FOLDER) {
            let volume = list_dir(
                get_manga_path(l.to_string().as_str(), title.as_str()),
                FileTypes::FOLDER,
            );

            manga_list.push(MangaThumbnail {
                title: title.clone(),
                lang: l.to_string(),
                img: get_image(
                    get_volume_path(
                        l.to_string().as_str(),
                        title.as_str(),
                        volume.first().expect("Could not find thumbnail").as_str(),
                    )
                    .as_str(),
                    0,
                ),
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
pub fn get_page(lang: &str, title: &str, volume: &str, page: usize) -> Json<Vec<u8>> {
    Json(get_image(&get_volume_path(lang, title, volume), page))
}
