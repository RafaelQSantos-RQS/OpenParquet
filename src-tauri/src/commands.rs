use crate::db_logic;
use crate::models::{
    ColumnInfo, FileListMetadata, FileMetadata, MultiFileMetadata, PageData, QueryResult,
};
use crate::state::AppState;
use crate::validation::{
    validate_parquet_directory, validate_parquet_path, validate_parquet_paths,
    validate_source_path, validate_sql_query, SourceKind,
};
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub fn load_parquet_schema(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<Vec<ColumnInfo>, String> {
    validate_parquet_path(&file_path).map_err(|e| e.to_string())?;

    let conn = state.conn.lock();
    db_logic::get_schema_from_db(&conn, &file_path).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_page_data(
    file_path: String,
    page: usize,
    page_size: usize,
    sort_col: Option<String>,
    sort_order: Option<String>,
    state: State<'_, AppState>,
) -> Result<PageData, String> {
    validate_parquet_path(&file_path).map_err(|e| e.to_string())?;

    let conn = state.conn.lock();
    let offset = page * page_size;

    let schema = db_logic::get_schema_from_db(&conn, &file_path).map_err(|e| e.to_string())?;
    let col_names: Vec<String> = schema.into_iter().map(|col| col.name).collect();

    db_logic::get_page_data_from_db(
        &conn, &file_path, col_names, page_size, offset, sort_col, sort_order,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_file_metadata(
    file_path: String,
    state: State<'_, AppState>,
) -> Result<FileMetadata, String> {
    validate_parquet_path(&file_path).map_err(|e| e.to_string())?;

    let conn = state.conn.lock();

    let schema = db_logic::get_schema_from_db(&conn, &file_path).map_err(|e| e.to_string())?;
    let total_rows =
        db_logic::get_row_count_from_db(&conn, &file_path).map_err(|e| e.to_string())?;

    Ok(FileMetadata {
        file_path,
        total_rows,
        schema,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub fn run_sql(
    source_path: String,
    query: String,
    page: usize,
    page_size: usize,
    file_paths: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<QueryResult, String> {
    validate_sql_query(&query).map_err(|e| e.to_string())?;

    let source_kind = match file_paths {
        Some(paths) => validate_parquet_paths(&paths).map_err(|e| e.to_string())?,
        None => validate_source_path(&source_path).map_err(|e| e.to_string())?,
    };

    let conn = state.conn.lock();
    let offset = page * page_size;

    match source_kind {
        SourceKind::File(path) => {
            db_logic::exec_custom_query(&conn, &path, &query, page_size, offset)
        }
        SourceKind::Directory(path) => {
            db_logic::exec_multi_file_query(&conn, &path, &query, page_size, offset)
        }
        SourceKind::FileList(paths) => {
            db_logic::exec_file_list_query(&conn, &paths, &query, page_size, offset)
        }
    }
    .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub fn export_data(
    source_path: String,
    query: String,
    output_path: String,
    format: String,
    file_paths: Option<Vec<String>>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    validate_sql_query(&query).map_err(|e| e.to_string())?;

    let source_kind = match file_paths {
        Some(paths) => validate_parquet_paths(&paths).map_err(|e| e.to_string())?,
        None => validate_source_path(&source_path).map_err(|e| e.to_string())?,
    };

    let conn = state.conn.lock();

    match source_kind {
        SourceKind::File(path) => {
            db_logic::export_query_to_file(&conn, &path, &query, &output_path, &format)
        }
        SourceKind::Directory(path) => {
            db_logic::export_multi_file_query(&conn, &path, &query, &output_path, &format)
        }
        SourceKind::FileList(paths) => {
            db_logic::export_file_list_query(&conn, &paths, &query, &output_path, &format)
        }
    }
    .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_multi_file_metadata(
    directory_path: String,
    state: State<'_, AppState>,
) -> Result<MultiFileMetadata, String> {
    validate_parquet_directory(&directory_path).map_err(|e| e.to_string())?;

    let conn = state.conn.lock();
    db_logic::get_multi_file_metadata(&conn, &directory_path).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_file_list_metadata(
    file_paths: Vec<String>,
    state: State<'_, AppState>,
) -> Result<FileListMetadata, String> {
    validate_parquet_paths(&file_paths).map_err(|e| e.to_string())?;

    let conn = state.conn.lock();
    db_logic::get_file_list_metadata(&conn, &file_paths).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "camelCase")]
pub fn get_multi_file_page_data(
    directory_path: String,
    page: usize,
    page_size: usize,
    sort_col: Option<String>,
    sort_order: Option<String>,
    state: State<'_, AppState>,
) -> Result<PageData, String> {
    validate_parquet_directory(&directory_path).map_err(|e| e.to_string())?;

    let conn = state.conn.lock();
    let offset = page * page_size;

    let schema =
        db_logic::get_multi_file_schema(&conn, &directory_path).map_err(|e| e.to_string())?;
    let col_names: Vec<String> = schema.into_iter().map(|col| col.name).collect();

    db_logic::get_multi_file_page_data(
        &conn,
        &directory_path,
        col_names,
        page_size,
        offset,
        sort_col,
        sort_order,
    )
    .map_err(|e| e.to_string())
}
