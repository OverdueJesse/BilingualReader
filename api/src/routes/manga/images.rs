use std::{
    fs::{self, File},
    io::Read,
};

pub fn get_image(path: &str, page: i8) -> Vec<u8> {
    let mut file = File::open(
        String::from(path) + "/" + &find_img_path(path, page),
    )
    .expect("Could not find file");

    let metadata = file.metadata().expect("Could not find img metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn find_img_path(path: &str, page: i8) -> String {
    let mut paths = fs::read_dir(path).unwrap();
    String::from(
        paths
            .nth(page as usize)
            .expect("Could not convert to usize")
            .expect("Could not find page")
            .file_name()
            .to_str()
            .expect("Failed to convert path to String"),
    )
}
