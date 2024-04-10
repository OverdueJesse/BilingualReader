mod helpers;
mod structs;
use self::{helpers::get_single_manga, structs::MangaThumbnail};
use rocket::serde::json::Json;
use structs::{ApiTest, Manga};

#[get("/")]
pub fn index() -> Json<Vec<ApiTest>> {
    Json(vec![])
}

#[get("/<lang>")]
pub fn view_manga(lang: &str) -> Json<Vec<MangaThumbnail>> {
    let langs: Vec<&str> = lang.split('-').collect();
    let mut manga_list: Vec<MangaThumbnail> = vec![];
    for l in langs {
        manga_list.append(&mut helpers::list_dir(l));
    }

    Json(manga_list)
}

#[get("/<lang>/<title>")]
pub fn get_manga(lang: &str, title: &str) -> Json<Manga> {
    let _manga = get_single_manga(lang, title);
    Json(Manga {
        title: title.to_string(),
        description: lang.to_string(),
        lang: lang.to_string(),
    })
}
