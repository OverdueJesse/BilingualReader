use crate::routes::manga::structs::{Manga, Metadata, Volume, VolumeList, VolumeMetadata};
use std::fs::{self, File};
use zip::{result::ZipResult, ZipArchive};
use crate::routes::manga::images::get_image;
use crate::common::{file_types::FileTypes, lang::Lang};

const MANGA_ROUTE: &str = "./resources/manga";

fn _zip_to_struct(path: &str) -> ZipResult<()> {
    println!("path : {}", path);
    let file = File::open(path)?;
    let zip = ZipArchive::new(file);

    for file in zip.iter() {
        println!("here : {}", file.len());
    }
    Ok(())
}

macro_rules! concat_path {
    (
        [ $( $x:expr ),* ],($delim:expr)
    ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec.join($delim)
        }
    };
}

pub fn get_all_manga() -> Vec<Manga> {
    let mut manga_list = vec![];

    for title in list_dir(MANGA_ROUTE.to_string(), FileTypes::FOLDER) {
        manga_list.push(match get_metadata(title) {
            Some(m) => m,
            None => continue
        });
    }

    manga_list
}

pub fn get_single_manga(title: String) -> VolumeList {
    let mut volume_list: VolumeList = VolumeList {
        en: vec![],
        jp: vec![],
    };

    let path = concat_path!([MANGA_ROUTE.to_string(), title.clone()], ("/"));
    for l in list_dir(path.clone(), FileTypes::FOLDER) {
        for v in list_dir(
            path.clone() + "/" + &*l,
            FileTypes::FOLDER,
        ) {
            let volume = Volume {
                title: v.clone(),
                metadata: get_volume_metadata(concat_path!([path.clone(), l.clone(), v.clone()], ("/"))),
            };

            match l.as_str() {
                "English" => volume_list.en.push(volume),
                "日本語" => volume_list.jp.push(volume),
                _ => continue,
            };
        }
    }

    volume_list
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

pub fn get_nth_volume(lang: Lang, title: &str, n: usize) -> Option<String> {
    let path = concat_path!([MANGA_ROUTE.to_string(), title.to_string(), lang.as_path().to_string()], ("/"));
    let paths = list_dir(path.clone(), FileTypes::FOLDER);
    match paths.get(n) {
        Some(s) => Some(concat_path!([path, s.clone()], ("/"))),
        None => None,
    }
}

pub fn get_metadata(title: String) -> Option<Manga> {
    let path = concat_path!([MANGA_ROUTE.to_owned(),  (&*title).to_string(), ".metadata.json".to_string()], ("/"));
    println!("{path}");
    let file = match File::open(path) {
        Ok(f) => f,
        Err(_e) => return None,
    };

    let metadata: Metadata = match serde_json::from_reader(file) {
        Ok(r) => r,
        Err(_e) => return None,
    };

    let img = concat_path!([MANGA_ROUTE.to_string(), metadata.thumbnail], (""));

    Some(Manga {
        title: metadata.title,
        volume_count: metadata.volume_count,
        thumbnail: get_image(&img, 0),
    })
}

pub fn get_volume_metadata(mut path: String) -> VolumeMetadata {
    let blank_metadata = VolumeMetadata {
        page_count: -1,
    };

    path = concat_path!(
        [
            path.clone(),
            ".metadata.json".to_string()
        ], ("/"));

    println!("path: {path}");

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_e) => return blank_metadata,
    };

    match serde_json::from_reader(file) {
        Ok(r) => r,
        Err(_e) => return blank_metadata,
    }
}
