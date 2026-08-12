use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

/// Data source contract sent by the frontend.
/// Serializa como `{ "type": "file", "path" } | { "type": "dir", "path" } | { "type": "list", "paths" }`
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SourceDescriptor {
    File { path: String },
    Dir { path: String },
    List { paths: Vec<String> },
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParquetFileInfo {
    pub file_name: String,
    pub file_path: String,
    pub row_count: i64,
}

/// Metadata of an opened dataset (file, folder or file list).
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DatasetInfo {
    pub schema: Vec<ColumnInfo>,
    pub total_rows: i64,
    pub files: Vec<ParquetFileInfo>,
}

pub type PageData = Vec<HashMap<String, String>>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    pub schema: Vec<ColumnInfo>,
    pub rows: Vec<HashMap<String, String>>,
    pub execution_time_ms: u64,
    pub total_rows: i64,
}
