use std::path::Path;
use std::fs;
use pdf_extract;
use crate::config::PLAIN_TEXT_EXTENSIONS;

pub fn extract_text_content(path: &Path, file_type: &str) -> String {
    let lowercase_file_type = file_type.to_lowercase();

    if PLAIN_TEXT_EXTENSIONS.contains(&lowercase_file_type.as_str()) {
        return extract_text_from_plain_text_file(path);
    }

    match file_type {
        "pdf" => extract_text_from_pdf(path),
        _ => String::new(),
    }
}

fn extract_text_from_pdf(path: &Path) -> String {
    pdf_extract::extract_text(path).unwrap_or_else(|e| {
        eprintln!("Erreur lors de l'extraction de texte du PDF {}: {}", path.display(), e);
        String::new()
    })
}

fn extract_text_from_plain_text_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Erreur de lecture du fichier texte {} ({}): {}", 
            path.display(), 
            path.extension().unwrap_or_default().to_str().unwrap_or_default(), 
            e
        );
        String::new()
    })
}