mod helpers;
mod structs;
use self::helpers::get_single_manga;
use rocket::serde::json::Json;
use structs::{ApiTest, Manga};

#[get("/")]
pub fn index() -> Json<Vec<ApiTest>> {
    Json(vec![])
}

#[get("/<lang>")]
pub fn view_manga(lang: &str) -> Json<Vec<ApiTest>> {
    let langs: Vec<&str> = lang.split('-').collect();
    let mut manga_list: Vec<String> = vec![];
    for l in langs {
        manga_list.append(&mut helpers::list_dir(l));
    }

    let mut res: Vec<ApiTest> = vec![];
    for n in manga_list.iter() {
        res.push(ApiTest {
            title: n.to_string(),
            description: "Test Desc".to_string(),
        });
    }

    Json(res)
}

#[get("/<lang>/<title>")]
pub fn get_manga(lang: &str, title: &str) -> Json<Manga> {
    let _manga = get_single_manga(lang, title);
    Json(Manga {
        title: title.to_string(),
        description: lang.to_string(),
    })
}

#[get("/test")]
pub fn test() -> Json<ApiTest> {
    return Json(ApiTest {
        title: String::from("Hello"),
        description: String::from("Test"),
    });
}
