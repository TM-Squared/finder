use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::io::{self, Write};
use clap::Parser;

mod config;
mod document;
mod extractor;
mod indexer;
mod searcher;
mod utils;


use crate::document::IndexedDocument;
use crate::indexer::index_documents;
use crate::searcher::search_index;
use crate::utils::list_files_in_directory;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Dossiers à indexer (séparés par des espaces)
    #[clap(required = false)]
    folders: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!("Démarrage de Finder");

    let folders = if args.folders.is_empty() {
        ask_for_folders()
    } else {
        args.folders
    };
    
    let index_path = PathBuf::from("./tantivy_index");

    // Spinner pour le parcours
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"])
            .template("{spinner} {msg}").unwrap()
    );
    spinner.set_message("🔍 Parcours des fichiers...");

    let documents: Vec<IndexedDocument> = folders.iter()
        .flat_map(|f| list_files_in_directory(f))
        .collect();
    spinner.finish_with_message("✅ Parcours terminé");

    // Afficher le nombre AVANT de passer la référence
    println!("📝 Documents à indexer : {}", documents.len());

    // Barre de progression
    let progress_bar = ProgressBar::new(documents.len() as u64);
    progress_bar.set_style(
        ProgressStyle::with_template("{wide_bar} {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("█▓▒░")
    );

    // Passe une référence immuable (&documents)
    if let Err(e) = index_documents(&index_path, &documents, |_| progress_bar.inc(1)) {
        progress_bar.finish_and_clear();
        eprintln!("❌ Erreur d'indexation : {}", e);
        return;
    }

    progress_bar.finish_with_message("✅ Indexation terminée");
    interactive_search(&index_path);
}

fn interactive_search(index_path: &PathBuf) {
    println!("\n--- Mode recherche (tapez 'quitter' pour sortir) ---");
    
    let stdin = io::stdin();
    loop {
        print!("Recherche > ");
        io::stdout().flush().unwrap();

        let mut query = String::new();
        stdin.read_line(&mut query).unwrap();
        let query = query.trim();

        if query.eq_ignore_ascii_case("quitter") {
            break;
        }

        if query.is_empty() {
            continue;
        }

        if let Err(e) = search_index(index_path, query) {
            eprintln!("Erreur de recherche : {}", e);
        }
    }
}

fn ask_for_folders() -> Vec<String> {
    println!("Entrez les chemins des dossiers à indexer (séparés par des espaces) :");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace().map(|s| s.to_string()).collect()
}