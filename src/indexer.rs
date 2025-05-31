use tantivy::schema::{Schema, TEXT, STORED, STRING};
use tantivy::{Index, doc};
use std::path::Path;
use crate::document::IndexedDocument;

pub fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("path", TEXT | STORED);
    schema_builder.add_text_field("filename", TEXT | STORED);
    schema_builder.add_text_field("content", TEXT);
    schema_builder.add_text_field("file_type", STRING | STORED);
    schema_builder.add_u64_field("size", STORED);
    schema_builder.build()
}

pub fn index_documents(
    index_path: &Path, 
    documents: &[IndexedDocument],
    progress_callback: impl Fn(usize)
) -> tantivy::Result<()> {
    let schema = create_schema();
    let index = create_or_open_index(index_path, &schema)?;
    
    let mut writer = index.writer(50_000_000)?;
    let fields = get_schema_fields(&schema)?;

    for (i, doc_to_index) in documents.iter().enumerate() {  // Utilisez iter() au lieu de into_iter()
        add_document_to_index(&mut writer, &fields, doc_to_index)?;  // Passe la référence
        progress_callback(i)
    }

    writer.commit()?;
    Ok(())
}

fn create_or_open_index(index_path: &Path, schema: &Schema) -> tantivy::Result<Index> {
    if index_path.exists() {
        Index::open_in_dir(index_path)
    } else {
        std::fs::create_dir_all(index_path)?;
        Index::create_in_dir(index_path, schema.clone())
    }
}

fn get_schema_fields(schema: &Schema) -> tantivy::Result<SchemaFields> {
    Ok(SchemaFields {
        path: schema.get_field("path")?,
        filename: schema.get_field("filename")?,
        content: schema.get_field("content")?,
        file_type: schema.get_field("file_type")?,
        size: schema.get_field("size")?,
    })
}

fn add_document_to_index(
    writer: &mut tantivy::IndexWriter,
    fields: &SchemaFields,
    doc: &IndexedDocument,  // Prend une référence maintenant
) -> tantivy::Result<()> {
    let tantivy_doc = doc!(
        fields.path => doc.get_path_as_string(),
        fields.filename => doc.filename.clone(),  // Clone nécessaire pour les String
        fields.content => doc.content.clone(),
        fields.file_type => doc.file_type.clone(),
        fields.size => doc.size
    );
    writer.add_document(tantivy_doc)?;
    Ok(())
}

struct SchemaFields {
    path: tantivy::schema::Field,
    filename: tantivy::schema::Field,
    content: tantivy::schema::Field,
    file_type: tantivy::schema::Field,
    size: tantivy::schema::Field,
}