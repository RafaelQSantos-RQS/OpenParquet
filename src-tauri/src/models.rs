// src-tauri/src/models.rs
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Serialize, Debug)]
pub struct FileMetadata {
    pub file_path: String,
    pub total_rows: i64,
    pub schema: Vec<ColumnInfo>,
}

pub type PageData = Vec<HashMap<String, String>>;