use tantivy::{Index, TantivyDocument};
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use tantivy::directory::MmapDirectory;
use tantivy::schema::Value;
use std::path::Path;


pub fn search_index(index_path: &Path, query_string: &str) -> tantivy::Result<()> {
    let index = Index::open(MmapDirectory::open(index_path)?)?;
    let schema = index.schema();
    let reader = index.reader()?;
    let searcher = reader.searcher();

    let fields = get_search_fields(&schema)?;
    let query = build_query(&index, &fields, query_string)?;
    
    let results = searcher.search(&query, &TopDocs::with_limit(10))?;
    display_results(&searcher, &schema, results, query_string);

    Ok(())
}

struct SearchFields {
    filename: tantivy::schema::Field,
    content: tantivy::schema::Field,
    #[allow(dead_code)]
    path: tantivy::schema::Field,
    #[allow(dead_code)]
    file_type: tantivy::schema::Field,
}

fn get_search_fields(schema: &tantivy::schema::Schema) -> tantivy::Result<SearchFields> {
    Ok(SearchFields {
        filename: schema.get_field("filename")?,
        content: schema.get_field("content")?,
        path: schema.get_field("path")?,
        file_type: schema.get_field("file_type")?,
    })
}

/// Construit l'objet Query à partir de la chaîne de recherche
fn build_query(
    index: &Index,
    fields: &SearchFields,
    query_string: &str,
) -> tantivy::Result<Box<dyn tantivy::query::Query>> {
    let query_parser = QueryParser::for_index(index, vec![fields.filename, fields.content]);
    Ok(query_parser.parse_query(query_string)?)
}

/// Affiche les résultats de la recherche
fn display_results(
    searcher: &tantivy::Searcher,
    schema: &tantivy::schema::Schema,
    results: Vec<(f32, tantivy::DocAddress)>,
    query: &str,
) {
    if results.is_empty() {
        println!("Aucun résultat trouvé pour \"{}\".", query);
        return;
    }

    println!("Résultats pour \"{}\" :", query);
    for (score, doc_address) in results {
        if let Ok(doc) = searcher.doc::<TantivyDocument>(doc_address) {
            display_document(schema, &doc, score);
        }
    }
}

/// Affiche les détails d'un document trouvé
fn display_document(
    schema: &tantivy::schema::Schema,
    doc: &TantivyDocument,
    score: f32,
) {
    let path = get_field_value(schema, doc, "path");
    let filename = get_field_value(schema, doc, "filename");
    let file_type = get_field_value(schema, doc, "file_type");

    println!("  Score: {:.2} - Fichier: {} (Type: {}, Chemin: {})", 
            score, filename, file_type, path);
}

/// Récupère la valeur textuelle d'un champ dans un document
fn get_field_value<'a>(
    schema: &tantivy::schema::Schema,
    doc: &'a TantivyDocument,
    field_name: &str,
) -> &'a str {
    match schema.get_field(field_name) {
        Ok(field) => {
            doc.get_first(field)
                .and_then(|v| v.as_str())
                .unwrap_or("N/A")
        },
        Err(_) => "N/A"
    }
}