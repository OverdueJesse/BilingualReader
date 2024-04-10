use crate::routes::manga::structs::Manga;
use std::fs::{self, File};
use zip::{result::ZipResult, ZipArchive};

use super::structs::MangaThumbnail;

fn zip_to_struct(path: &str) -> ZipResult<()> {
    let file = File::open(path)?;
    let zip = ZipArchive::new(file);

    for file in zip.iter() {
        println!("{}", file.len());
    }
    Ok(())
}

fn get_lang_path(lang: &str) -> String {
    match lang {
        "jp" => "B://漫画/日本語".to_string(),
        "en" => "B://漫画/English".to_string(),
        _ => panic!("Language not available"),
    }
}

fn get_manga_path(lang: &str, title: &str) -> String {
    get_lang_path(lang) + "/" + title
}

pub fn get_single_manga(lang: &str, title: &str) -> Manga {
    let path = get_manga_path(lang, title);
    let _test = zip_to_struct(path.as_str());
    Manga {
        title: path,
        description: "test".to_string(),
        lang: lang.to_string(),
    }
}

pub fn list_dir(lang: &str) -> Vec<MangaThumbnail> {
    let path = get_lang_path(lang);

    let paths = fs::read_dir(path).unwrap();
    let mut manga_list: Vec<MangaThumbnail> = vec![];

    for path in paths {
        let pathname = String::from(
            path.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .expect("Failed to convert path"),
        );
        if !pathname.contains(".") {
            manga_list.push(MangaThumbnail {
                title: pathname,
                lang: lang.to_string()
            });
        }
    }

    return manga_list;
}
