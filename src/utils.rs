use std::path::Path;
use walkdir::WalkDir;
use crate::config::{IGNORED_DIRS, IGNORED_EXTENSIONS};
use crate::document::IndexedDocument;
use crate::extractor::extract_text_content;

pub fn should_ignore(path: &Path) -> bool {
    path.iter()
        .filter_map(|c| c.to_str())
        .any(|s| IGNORED_DIRS.contains(&s)) ||
    (path.is_file() && 
     path.extension()
        .and_then(|e| e.to_str())
        .map(|e| IGNORED_EXTENSIONS.contains(&e.to_lowercase().as_str()))
        .unwrap_or(false))
}

pub fn list_files_in_directory(directory: &str) -> Vec<IndexedDocument> {
    WalkDir::new(directory)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !should_ignore(e.path()))
        .filter_map(process_file_entry)
        .collect()
}

fn process_file_entry(entry: walkdir::DirEntry) -> Option<IndexedDocument> {
    if !entry.path().is_file() {
        return None;
    }

    let metadata = entry.metadata().ok()?;
    let path = entry.path().to_path_buf();
    
    Some(IndexedDocument::new(
        path.clone(),
        entry.file_name().to_str()?.to_string(),
        extract_text_content(&path, &get_file_extension(&path)),
        metadata.modified().unwrap_or(std::time::UNIX_EPOCH),
        metadata.len(),
        get_file_extension(&path),
    ))
}

fn get_file_extension(path: &std::path::Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string()
}