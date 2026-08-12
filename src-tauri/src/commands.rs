use crate::db_logic;
use crate::error::AppResult;
use crate::models::{DatasetInfo, PageData, QueryResult, SourceDescriptor};
use crate::state::AppState;
use crate::validation::{validate_source, validate_sql_query};
use tauri::State;

/// Opens a dataset (file, folder or file list) and returns metadata.
#[tauri::command(rename_all = "camelCase")]
pub fn open_dataset(
    source: SourceDescriptor,
    state: State<'_, AppState>,
) -> AppResult<DatasetInfo> {
    validate_source(&source)?;
    let conn = state.conn.lock();
    Ok(db_logic::get_dataset_info(&conn, &source)?)
}

/// Returns a page of dataset data, with optional sorting.
#[tauri::command(rename_all = "camelCase")]
pub fn get_page(
    source: SourceDescriptor,
    page: usize,
    page_size: usize,
    sort_col: Option<String>,
    sort_order: Option<String>,
    state: State<'_, AppState>,
) -> AppResult<PageData> {
    validate_source(&source)?;
    let conn = state.conn.lock();

    let schema = db_logic::get_schema(&conn, &source)?;
    let col_names: Vec<String> = schema.into_iter().map(|col| col.name).collect();

    Ok(db_logic::get_page(
        &conn,
        &source,
        &col_names,
        page_size,
        page * page_size,
        sort_col.as_deref(),
        sort_order.as_deref(),
    )?)
}

/// Runs an arbitrary SQL query over the dataset and returns the first page.
#[tauri::command(rename_all = "camelCase")]
pub fn run_sql(
    source: SourceDescriptor,
    query: String,
    page: usize,
    page_size: usize,
    state: State<'_, AppState>,
) -> AppResult<QueryResult> {
    validate_source(&source)?;
    validate_sql_query(&query)?;
    let conn = state.conn.lock();
    Ok(db_logic::exec_query(&conn, &source, &query, page_size, page * page_size)?)
}

/// Exports a query result to a file (CSV/JSON/PARQUET).
#[tauri::command(rename_all = "camelCase")]
pub fn export_dataset(
    source: SourceDescriptor,
    query: String,
    output_path: String,
    format: String,
    state: State<'_, AppState>,
) -> AppResult<()> {
    validate_source(&source)?;
    validate_sql_query(&query)?;
    let conn = state.conn.lock();
    Ok(db_logic::export_query(&conn, &source, &query, &output_path, &format)?)
}
