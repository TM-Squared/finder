use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct IndexedDocument {
    pub path: PathBuf,
    pub filename: String,
    pub content: String,
    #[allow(dead_code)]
    pub last_modified: SystemTime,
    pub size: u64,
    pub file_type: String,
}

impl IndexedDocument {
    pub fn new(
        path: PathBuf,
        filename: String,
        content: String,
        last_modified: SystemTime,
        size: u64,
        file_type: String,
    ) -> Self {
        Self {
            path,
            filename,
            content,
            last_modified,
            size,
            file_type,
        }
    }

    pub fn get_path_as_string(&self) -> String {
        self.path.display().to_string()
    }
}