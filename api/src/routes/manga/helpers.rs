use crate::routes::manga::structs::{MangaThumbnail};
use std::fs::{self, File};
use zip::{result::ZipResult, ZipArchive};
use crate::routes::manga::images::get_image;
use crate::common::{file_types::FileTypes, lang::Lang};

fn _zip_to_struct(path: &str) -> ZipResult<()> {
    println!("path : {}", path);
    let file = File::open(path)?;
    let zip = ZipArchive::new(file);

    for file in zip.iter() {
        println!("here : {}", file.len());
    }
    Ok(())
}

pub fn get_lang_path(lang: Lang) -> Option<String> {
    match lang {
        Lang::JP => Some(String::from("./resources/manga/日本語")),
        Lang::EN => Some(String::from("./resources/manga/English")),
    }
}

pub fn get_manga_path(lang: Lang, title: &str) -> Option<String> {
    match get_lang_path(lang) {
        Some(s) => Some(s + "/" + title),
        None => None
    }
}

pub fn get_volume_path(lang: Lang, title: &str, volume: &str) -> Option<String> {
    match get_manga_path(lang, title) {
        Some(s) => Some(s + "/" + volume),
        None => None,
    }
}

pub fn get_nth_volume(lang: Lang, title: &str, n: usize) -> Option<String> {
    let paths = match get_single_manga(lang, title) {
        Some(v) => v,
        None => return None,
    };

    for (i,  volume) in paths.iter().enumerate() {
        if i == n {
            return get_volume_path(lang, title, volume);
        }
    }

    None
}

pub fn get_single_manga(lang: Lang, title: &str) -> Option<Vec<String>> {
    match get_manga_path(lang, title) {
        Some(s) => Some(list_dir(s, FileTypes::FOLDER)),
        None => return None,
    }
}

pub fn list_dir(path: String, file_type: FileTypes) -> Vec<String> {
    let mut entries: Vec<String> = vec![];

    for path in fs::read_dir(path).unwrap() {
        let pathname = String::from(
            path.as_ref()
                .unwrap()
                .file_name()
                .to_str()
                .expect("Failed to convert path"),
        );
       if file_type.to_bool(&pathname) {
            entries.push(pathname);
        } 
    }

    entries
}

pub fn create_manga_thumbnail(title: String, l: Lang) -> MangaThumbnail {
    let blank_thumbnail = MangaThumbnail {
        title: String::from("ERROR"),
        lang: "".to_string(),
        img: vec![],
    };

    let volume = match get_nth_volume(l, title.as_str(), 0) {
        Some(s) => s,
        None => return blank_thumbnail,
    };

    MangaThumbnail {
        title: title.clone(),
        lang: l.to_string(),
        img: get_image(volume.as_str(),0),
    }
}
