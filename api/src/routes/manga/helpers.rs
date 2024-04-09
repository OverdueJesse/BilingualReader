use crate::routes::manga::structs::Manga;
// use std::io::prelude::*;
// use zip::{result::ZipResult, ZipArchive};
use std::fs;

// fn zip_to_struct(reader: impl Read + Seek) -> ZipResult<()> {
//     let mut zip = ZipArchive::new(reader)?;

//     for i in 0..zip.len() {
//         let mut file = zip.by_index(i)?;
//         println!("Filename: {}", file.name());
//         let _ = std::io::copy(&mut file, &mut std::io::stdout());
//     }

//     Ok(())
// }

fn get_lang_path (lang: &str) -> String {
    match lang {
        "jp" => "B://漫画/日本語".to_string(),
        "en" => "B://漫画/English".to_string(),
        _ => panic!("Language not available"),
    }
}

fn get_manga_path (lang: &str, title: &str) -> String {
    let mut path = get_lang_path(lang);
    path.push_str(title);
    path
}

pub fn get_single_manga(lang: &str, title: &str) -> Manga {
    let _path = get_manga_path(lang, title);
    Manga {
        title: title.to_string(),
        description: lang.to_string(),
    }
}

pub fn list_dir(lang: &str) -> Vec<String> {
    let path = get_lang_path(lang);

    let paths = fs::read_dir(path).unwrap();
    let mut manga_list: Vec<String> = vec![];

    for path in paths {
        let pathname = String::from(
            path.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .expect("Failed to convert path"),
        );
        if !pathname.contains(".ini") {
            manga_list.push(pathname);
        }
    }

    let mut res: Vec<String> = vec![];
    for manga in manga_list.iter() {
        res.push(manga.to_string());
    }

    return res;
}
