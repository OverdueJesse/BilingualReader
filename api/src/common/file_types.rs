pub enum FileTypes {
    CBZ,
    ZIP,
    FOLDER,
    METADATA,
    IMAGE,
}

impl FileTypes {
    pub fn to_bool(&self, s: &str) -> bool {
        match self {
            FileTypes::CBZ => s.contains(".cbz"),
            FileTypes::ZIP => s.contains(".zip"),
            FileTypes::METADATA => s.eq(".metadata.json"),
            FileTypes::IMAGE => s.contains(".jpg") || s.contains(".png"),
            FileTypes::FOLDER => !s.contains("."),
        }
    }

    pub fn _to_string(&self) -> String {
        match self {
            FileTypes::CBZ => String::from(".cbz"),
            FileTypes::ZIP => String::from(".zip"),
            FileTypes::FOLDER => String::from("Folder"),
            FileTypes::METADATA => String::from("Metadata"),
            FileTypes::IMAGE => String::from("Image"),
        }
    }
}