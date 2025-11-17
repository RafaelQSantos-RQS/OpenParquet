// src-tauri/commands.rs
use crate::db_logic;
use crate::models::{ColumnInfo, FileMetadata, PageData};
use duckdb::Connection;

fn db_err(e: duckdb::Error) -> String {
    e.to_string()
}

#[tauri::command(rename_all = "camelCase")]
pub async fn load_parquet_schema(file_path: String) -> Result<Vec<ColumnInfo>, String> {
    println!("Backend: Lendo esquema de {}", file_path);
    let conn = Connection::open_in_memory().map_err(db_err)?;
    
    // Chama a lógica de DB
    db_logic::get_schema_from_db(&conn, &file_path).map_err(db_err)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_page_data(
    file_path: String,
    page: usize,
    page_size: usize,
) -> Result<PageData, String> {
    
    println!("Backend: Lendo dados para: {}, página {}", file_path, page);
    let conn = Connection::open_in_memory().map_err(db_err)?;
    let offset = page * page_size;

    let schema = db_logic::get_schema_from_db(&conn, &file_path).map_err(db_err)?;
    let col_names: Vec<String> = schema.into_iter().map(|col| col.name).collect();
    
    let data = db_logic::get_page_data_from_db(&conn, &file_path, col_names, page_size, offset)
        .map_err(db_err)?;
    
    println!("Backend: Retornando {} linhas para o Svelte.", data.len());
    Ok(data)
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_file_metadata(file_path: String) -> Result<FileMetadata, String> {
    println!("Backend: Lendo metadados de {}", file_path);
    let conn = Connection::open_in_memory().map_err(db_err)?;

    let schema = db_logic::get_schema_from_db(&conn, &file_path).map_err(db_err)?;
    let total_rows = db_logic::get_row_count_from_db(&conn, &file_path).map_err(db_err)?;

    println!("Backend: Metadados: {} colunas, {} linhas.", schema.len(), total_rows);
    
    Ok(FileMetadata {
        file_path,
        total_rows,
        schema,
    })
}