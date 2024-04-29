use std::{
    fs::{self, File},
    io::Read,
};

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

    let metadata = file.metadata().expect("Could not find img metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn find_img_path(path: &str, page: usize) -> String {
    let mut paths = fs::read_dir(path).unwrap();
    let page = match paths.nth(page) {
        Some(p) => p,
        None => return String::from(IMAGE_ERROR),
    };

    String::from(
        page.expect("Could not find page")
            .file_name()
            .to_str()
            .expect("Failed to convert path to String"),
    )
}
