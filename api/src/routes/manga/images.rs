use std::{
    fs::{self, File},
    io::Read,
};
use crate::common::file_types::FileTypes;
use crate::routes::manga::helpers::list_dir;

const IMAGE_ERROR: &str = "ERROR";

pub fn get_image(path: &str, page: usize) -> Vec<u8> {
    let file_path = String::from(path) + "/" + &find_img_path(path, page);

    if file_path == IMAGE_ERROR {
        return vec![];
    }

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_err) => return vec![],
    };

    let metadata = match file.metadata() {
        Ok(metadata) => metadata,
        Err(_err) => return vec![],
    };

    let mut buffer = vec![0; metadata.len() as usize];

    match file.read(&mut buffer){
        Ok(_ok) => buffer,
        Err(_err) => vec![],
    }
}

fn find_img_path(path: &str, page: usize) -> String {
    let mut paths = list_dir(path.to_string(), FileTypes::IMAGE);
    let page = match paths.iter().nth(page) {
        Some(p) => p,
        None => return String::from(IMAGE_ERROR),
    };

    page.to_string()
}
