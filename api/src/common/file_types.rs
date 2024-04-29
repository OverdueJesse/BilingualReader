pub enum FileTypes {
    CBZ,
    ZIP,
    FOLDER,
}

impl FileTypes {
    pub fn to_bool(&self, s: &str) -> bool {
        match self {
            FileTypes::CBZ => s.contains(".cbz"),
            FileTypes::ZIP => s.contains(".zip"),
            FileTypes::FOLDER => !s.contains("."),
        }
    }

    pub fn _to_string(&self) -> String {
        match self {
            FileTypes::CBZ => ".cbz".to_string(),
            FileTypes::ZIP => ".zip".to_string(),
            FileTypes::FOLDER => "Folder".to_string(),
        }
    }
}