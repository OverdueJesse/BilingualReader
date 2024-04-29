use crate::routes::manga::structs::FileTypes;
use std::fs::{self, File};
use zip::{result::ZipResult, ZipArchive};

fn _zip_to_struct(path: &str) -> ZipResult<()> {
    println!("path : {}", path);
    let file = File::open(path)?;
    let zip = ZipArchive::new(file);

    for file in zip.iter() {
        println!("here : {}", file.len());
    }
    Ok(())
}

pub fn get_lang_path(lang: &str) -> String {
    match lang {
        "jp" => "./resources/manga/日本語".to_string(),
        "en" => "./resources/manga/English".to_string(),
        _ => panic!("Language not available"),
    }
}

pub fn get_manga_path(lang: &str, title: &str) -> String {
    get_lang_path(lang) + "/" + title
}

pub fn get_volume_path(lang: &str, title: &str, volume: &str) -> String {
   get_manga_path(lang, title) + "/" + volume
}

pub fn get_single_manga(lang: &str, title: &str) -> Vec<String> {
    list_dir(get_manga_path(lang, title), FileTypes::FOLDER)
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
