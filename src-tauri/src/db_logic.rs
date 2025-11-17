// src-tauri/db_logic.rs
use crate::models::{ColumnInfo, PageData};
use duckdb::types::ValueRef;
use duckdb::{Connection, Result};
use std::collections::HashMap;

pub fn get_schema_from_db(conn: &Connection, file_path: &str) -> Result<Vec<ColumnInfo>> {
    let sql = format!("DESCRIBE SELECT * FROM '{}';", file_path);
    let mut stmt = conn.prepare(&sql)?;

    let columns_iter = stmt.query_map([], |row| {
        Ok(ColumnInfo {
            name: row.get(0)?,
            type_: row.get(1)?,
        })
    })?;

    let schema = columns_iter.filter_map(Result::ok).collect();
    Ok(schema)
}

pub fn get_row_count_from_db(conn: &Connection, file_path: &str) -> Result<i64> {
    let sql = format!("SELECT COUNT(*) FROM '{}';", file_path);
    let total_rows = conn.query_row(&sql, [], |row| row.get(0))?;
    Ok(total_rows)
}

pub fn get_page_data_from_db(
    conn: &Connection,
    file_path: &str,
    col_names: Vec<String>,
    limit: usize,
    offset: usize,
) -> Result<PageData> {
    
    let select_casts = col_names
        .iter()
        .map(|name| format!("\"{}\"::VARCHAR", name))
        .collect::<Vec<String>>()
        .join(", ");

    let sql = format!(
        "SELECT {} FROM '{}' LIMIT {} OFFSET {}",
        select_casts, file_path, limit, offset
    );

    let mut stmt = conn.prepare(&sql)?;

    let rows_iter = stmt.query_map([], |row| {
        let mut row_map = HashMap::new();
        for (i, name) in col_names.iter().enumerate() {
            let val_ref_result = row.get_ref(i);
            let val_str = match val_ref_result {
                Ok(val_ref) => match val_ref {
                    ValueRef::Null => "NULL".to_string(),
                    ValueRef::Text(bytes) => String::from_utf8_lossy(bytes).to_string(),
                    _ => "[Tipo inesperado]".to_string(),
                },
                Err(e) => {
                    println!("[ERRO DB] Falha ao ler coluna {}: {}", name, e);
                    "[ERRO]".to_string()
                }
            };
            row_map.insert(name.clone(), val_str);
        }
        Ok(row_map)
    })?;

    let data = rows_iter.filter_map(Result::ok).collect();
    Ok(data)
}