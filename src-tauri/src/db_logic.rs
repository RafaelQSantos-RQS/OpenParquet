// src-tauri/src/db_logic.rs
use crate::models::{
    ColumnInfo, FileListMetadata, MultiFileMetadata, PageData, ParquetFileInfo, QueryResult,
};
use chrono::NaiveDate;
use duckdb::types::ValueRef;
use duckdb::{Connection, Result};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

fn format_value(val: ValueRef) -> String {
    match val {
        ValueRef::Null => "NULL".to_string(),
        ValueRef::Boolean(b) => b.to_string(),
        ValueRef::TinyInt(i) => i.to_string(),
        ValueRef::SmallInt(i) => i.to_string(),
        ValueRef::Int(i) => i.to_string(),
        ValueRef::BigInt(i) => i.to_string(),
        ValueRef::HugeInt(i) => i.to_string(),
        ValueRef::UTinyInt(u) => u.to_string(),
        ValueRef::USmallInt(u) => u.to_string(),
        ValueRef::UInt(u) => u.to_string(),
        ValueRef::UBigInt(u) => u.to_string(),
        ValueRef::Float(f) => f.to_string(),
        ValueRef::Double(d) => d.to_string(),
        ValueRef::Decimal(d) => d.to_string(),
        ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
        ValueRef::Date32(days) => match NaiveDate::from_ymd_opt(1970, 1, 1) {
            Some(epoch) => match epoch.checked_add_signed(chrono::Duration::days(days as i64)) {
                Some(d) => d.format("%Y-%m-%d").to_string(),
                None => days.to_string(),
            },
            None => days.to_string(),
        },
        ValueRef::Time64(_, micros) => micros.to_string(),
        ValueRef::Timestamp(_, micros) => micros.to_string(),
        _ => format!("{:?}", val),
    }
}

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
    sort_col: Option<String>,
    sort_order: Option<String>,
) -> Result<PageData> {
    let select_casts = col_names
        .iter()
        .map(|name| format!("\"{}\"::VARCHAR", name))
        .collect::<Vec<String>>()
        .join(", ");

    let order_clause = if let (Some(col), Some(order)) = (sort_col, sort_order) {
        let direction = if order.to_uppercase() == "DESC" {
            "DESC"
        } else {
            "ASC"
        };
        format!("ORDER BY \"{}\" {}", col, direction)
    } else {
        String::new()
    };

    let sql = format!(
        "SELECT {} FROM '{}' {} LIMIT {} OFFSET {}",
        select_casts, file_path, order_clause, limit, offset
    );

    let mut stmt = conn.prepare(&sql)?;

    let rows_iter = stmt.query_map([], |row| {
        let mut row_map = HashMap::new();
        for (i, name) in col_names.iter().enumerate() {
            let val_str = match row.get_ref(i) {
                Ok(val) => format_value(val),
                Err(_) => "ERROR".to_string(),
            };
            row_map.insert(name.clone(), val_str);
        }
        Ok(row_map)
    })?;

    let data = rows_iter.filter_map(Result::ok).collect();
    Ok(data)
}

pub fn exec_custom_query(
    conn: &Connection,
    file_path: &str,
    user_query: &str,
    limit: usize,
    offset: usize,
) -> Result<QueryResult> {
    let start = Instant::now();

    let view_sql = format!("CREATE OR REPLACE VIEW t AS SELECT * FROM '{}';", file_path);
    conn.execute(&view_sql, [])?;

    let clean_query = user_query.trim().trim_end_matches(';');

    let count_sql = format!("SELECT COUNT(*) FROM ({})", clean_query);
    let total_rows: i64 = conn.query_row(&count_sql, [], |row| row.get(0))?;

    let describe_sql = format!("DESCRIBE SELECT * FROM ({})", clean_query);
    let mut stmt_desc = conn.prepare(&describe_sql)?;

    let schema_iter = stmt_desc.query_map([], |row| {
        Ok(ColumnInfo {
            name: row.get(0)?,
            type_: row.get(1)?,
        })
    })?;

    let mut schema = Vec::new();
    for col in schema_iter.flatten() {
        schema.push(col);
    }

    let paged_sql = format!(
        "SELECT * FROM ({}) LIMIT {} OFFSET {}",
        clean_query, limit, offset
    );
    let mut stmt = conn.prepare(&paged_sql)?;

    let rows_iter = stmt.query_map([], |row| {
        let mut row_map = HashMap::new();
        for (i, col) in schema.iter().enumerate() {
            let val_str = match row.get_ref(i) {
                Ok(val) => format_value(val),
                Err(_) => "NULL".to_string(),
            };
            row_map.insert(col.name.clone(), val_str);
        }
        Ok(row_map)
    })?;

    let rows = rows_iter.filter_map(Result::ok).collect();
    let duration = start.elapsed().as_millis() as u64;

    Ok(QueryResult {
        schema,
        rows,
        execution_time_ms: duration,
        total_rows,
    })
}

pub fn export_query_to_file(
    conn: &Connection,
    file_path: &str,
    query: &str,
    output_path: &str,
    format: &str,
) -> Result<()> {
    let view_sql = format!("CREATE OR REPLACE VIEW t AS SELECT * FROM '{}'", file_path);
    conn.execute(&view_sql, [])?;

    let (fmt_cmd, opts) = match format.to_uppercase().as_str() {
        "JSON" => ("JSON", "ARRAY true"),
        "PARQUET" => ("PARQUET", "COMPRESSION 'SNAPPY'"),
        _ => ("CSV", "HEADER true, DELIMITER ','"),
    };

    let clean_query = query.trim().trim_end_matches(";");

    let copy_sql = format!(
        "COPY ({}) TO '{}' (FORMAT {},{});",
        clean_query, output_path, fmt_cmd, opts
    );

    conn.execute(&copy_sql, [])?;
    Ok(())
}

pub fn list_parquet_files(conn: &Connection, dir_path: &str) -> Result<Vec<ParquetFileInfo>> {
    let glob_pattern = format!("{}/**/*.parquet", dir_path);
    let sql = format!(
        "SELECT filename, COUNT(*) as row_count FROM read_parquet('{}', filename=true) GROUP BY filename ORDER BY filename",
        glob_pattern
    );

    let mut stmt = conn.prepare(&sql)?;
    let files_iter = stmt.query_map([], |row| {
        let full_path: String = row.get(0)?;
        let row_count: i64 = row.get(1)?;
        let file_name = Path::new(&full_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| full_path.clone());

        Ok(ParquetFileInfo {
            file_name,
            file_path: full_path,
            row_count,
        })
    })?;

    Ok(files_iter.filter_map(Result::ok).collect())
}

pub fn get_multi_file_schema(conn: &Connection, dir_path: &str) -> Result<Vec<ColumnInfo>> {
    let glob_pattern = format!("{}/**/*.parquet", dir_path);
    let sql = format!(
        "DESCRIBE SELECT * FROM read_parquet('{}', filename=true);",
        glob_pattern
    );

    let mut stmt = conn.prepare(&sql)?;
    let columns_iter = stmt.query_map([], |row| {
        Ok(ColumnInfo {
            name: row.get(0)?,
            type_: row.get(1)?,
        })
    })?;

    Ok(columns_iter.filter_map(Result::ok).collect())
}

pub fn get_multi_file_metadata(conn: &Connection, dir_path: &str) -> Result<MultiFileMetadata> {
    let files = list_parquet_files(conn, dir_path)?;
    let schema = get_multi_file_schema(conn, dir_path)?;
    let total_rows: i64 = files.iter().map(|f| f.row_count).sum();

    Ok(MultiFileMetadata {
        directory_path: dir_path.to_string(),
        files,
        total_rows,
        schema,
    })
}

pub fn get_file_list_metadata(
    conn: &Connection,
    file_paths: &[String],
) -> Result<FileListMetadata> {
    let source_sql = build_source_sql(file_paths);

    let sql = format!(
        "SELECT filename, COUNT(*) as row_count FROM read_parquet({}, filename=true) GROUP BY filename ORDER BY filename",
        source_sql
    );

    let mut stmt = conn.prepare(&sql)?;
    let files_iter = stmt.query_map([], |row| {
        let full_path: String = row.get(0)?;
        let row_count: i64 = row.get(1)?;
        let file_name = Path::new(&full_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| full_path.clone());

        Ok(ParquetFileInfo {
            file_name,
            file_path: full_path,
            row_count,
        })
    })?;

    let files: Vec<ParquetFileInfo> = files_iter.filter_map(Result::ok).collect();
    let total_rows: i64 = files.iter().map(|f| f.row_count).sum();

    let schema_sql = format!(
        "DESCRIBE SELECT * FROM read_parquet({}, filename=true);",
        source_sql
    );
    let mut stmt_schema = conn.prepare(&schema_sql)?;
    let schema_iter = stmt_schema.query_map([], |row| {
        Ok(ColumnInfo {
            name: row.get(0)?,
            type_: row.get(1)?,
        })
    })?;
    let schema: Vec<ColumnInfo> = schema_iter.filter_map(Result::ok).collect();

    Ok(FileListMetadata {
        files,
        total_rows,
        schema,
    })
}

pub fn get_multi_file_page_data(
    conn: &Connection,
    dir_path: &str,
    col_names: Vec<String>,
    limit: usize,
    offset: usize,
    sort_col: Option<String>,
    sort_order: Option<String>,
) -> Result<PageData> {
    let glob_pattern = format!("{}/**/*.parquet", dir_path);

    let select_casts: String = col_names
        .iter()
        .map(|name| {
            if name == "filename" {
                "filename".to_string()
            } else {
                format!("\"{}\"::VARCHAR", name)
            }
        })
        .collect::<Vec<String>>()
        .join(", ");

    let order_clause = if let (Some(col), Some(order)) = (sort_col, sort_order) {
        let direction = if order.to_uppercase() == "DESC" {
            "DESC"
        } else {
            "ASC"
        };
        format!("ORDER BY \"{}\" {}", col, direction)
    } else {
        String::new()
    };

    let sql = format!(
        "SELECT {} FROM read_parquet('{}', filename=true) {} LIMIT {} OFFSET {}",
        select_casts, glob_pattern, order_clause, limit, offset
    );

    let mut stmt = conn.prepare(&sql)?;

    let rows_iter = stmt.query_map([], |row| {
        let mut row_map = HashMap::new();
        for (i, name) in col_names.iter().enumerate() {
            let val_str = match row.get_ref(i) {
                Ok(val) => format_value(val),
                Err(_) => "ERROR".to_string(),
            };
            row_map.insert(name.clone(), val_str);
        }
        Ok(row_map)
    })?;

    Ok(rows_iter.filter_map(Result::ok).collect())
}

pub fn exec_multi_file_query(
    conn: &Connection,
    dir_path: &str,
    user_query: &str,
    limit: usize,
    offset: usize,
) -> Result<QueryResult> {
    let start = Instant::now();
    let glob_pattern = format!("{}/**/*.parquet", dir_path);

    let view_sql = format!(
        "CREATE OR REPLACE VIEW t AS SELECT * FROM read_parquet('{}', filename=true);",
        glob_pattern
    );
    conn.execute(&view_sql, [])?;

    let clean_query = user_query.trim().trim_end_matches(';');

    let count_sql = format!("SELECT COUNT(*) FROM ({})", clean_query);
    let total_rows: i64 = conn.query_row(&count_sql, [], |row| row.get(0))?;

    let describe_sql = format!("DESCRIBE SELECT * FROM ({})", clean_query);
    let mut stmt_desc = conn.prepare(&describe_sql)?;

    let schema_iter = stmt_desc.query_map([], |row| {
        Ok(ColumnInfo {
            name: row.get(0)?,
            type_: row.get(1)?,
        })
    })?;

    let mut schema = Vec::new();
    for col in schema_iter.flatten() {
        schema.push(col);
    }

    let paged_sql = format!(
        "SELECT * FROM ({}) LIMIT {} OFFSET {}",
        clean_query, limit, offset
    );
    let mut stmt = conn.prepare(&paged_sql)?;

    let rows_iter = stmt.query_map([], |row| {
        let mut row_map = HashMap::new();
        for (i, col) in schema.iter().enumerate() {
            let val_str = match row.get_ref(i) {
                Ok(val) => format_value(val),
                Err(_) => "NULL".to_string(),
            };
            row_map.insert(col.name.clone(), val_str);
        }
        Ok(row_map)
    })?;

    let rows = rows_iter.filter_map(Result::ok).collect();
    let duration = start.elapsed().as_millis() as u64;

    Ok(QueryResult {
        schema,
        rows,
        execution_time_ms: duration,
        total_rows,
    })
}

pub fn export_multi_file_query(
    conn: &Connection,
    dir_path: &str,
    query: &str,
    output_path: &str,
    format: &str,
) -> Result<()> {
    let glob_pattern = format!("{}/**/*.parquet", dir_path);

    let view_sql = format!(
        "CREATE OR REPLACE VIEW t AS SELECT * FROM read_parquet('{}', filename=true)",
        glob_pattern
    );
    conn.execute(&view_sql, [])?;

    let (fmt_cmd, opts) = match format.to_uppercase().as_str() {
        "JSON" => ("JSON", "ARRAY true"),
        "PARQUET" => ("PARQUET", "COMPRESSION 'SNAPPY'"),
        _ => ("CSV", "HEADER true, DELIMITER ','"),
    };

    let clean_query = query.trim().trim_end_matches(";");

    let copy_sql = format!(
        "COPY ({}) TO '{}' (FORMAT {},{});",
        clean_query, output_path, fmt_cmd, opts
    );

    conn.execute(&copy_sql, [])?;
    Ok(())
}

fn build_source_sql(file_paths: &[String]) -> String {
    let paths_str: Vec<String> = file_paths.iter().map(|p| format!("'{}'", p)).collect();
    format!("[{}]", paths_str.join(", "))
}

pub fn exec_file_list_query(
    conn: &Connection,
    file_paths: &[String],
    user_query: &str,
    limit: usize,
    offset: usize,
) -> Result<QueryResult> {
    let start = Instant::now();
    let source_sql = build_source_sql(file_paths);

    let view_sql = format!(
        "CREATE OR REPLACE VIEW t AS SELECT * FROM read_parquet({}, filename=true);",
        source_sql
    );
    conn.execute(&view_sql, [])?;

    let clean_query = user_query.trim().trim_end_matches(';');

    let count_sql = format!("SELECT COUNT(*) FROM ({})", clean_query);
    let total_rows: i64 = conn.query_row(&count_sql, [], |row| row.get(0))?;

    let describe_sql = format!("DESCRIBE SELECT * FROM ({})", clean_query);
    let mut stmt_desc = conn.prepare(&describe_sql)?;

    let schema_iter = stmt_desc.query_map([], |row| {
        Ok(ColumnInfo {
            name: row.get(0)?,
            type_: row.get(1)?,
        })
    })?;

    let mut schema = Vec::new();
    for col in schema_iter.flatten() {
        schema.push(col);
    }

    let paged_sql = format!(
        "SELECT * FROM ({}) LIMIT {} OFFSET {}",
        clean_query, limit, offset
    );
    let mut stmt = conn.prepare(&paged_sql)?;

    let rows_iter = stmt.query_map([], |row| {
        let mut row_map = HashMap::new();
        for (i, col) in schema.iter().enumerate() {
            let val_str = match row.get_ref(i) {
                Ok(val) => format_value(val),
                Err(_) => "NULL".to_string(),
            };
            row_map.insert(col.name.clone(), val_str);
        }
        Ok(row_map)
    })?;

    let rows = rows_iter.filter_map(Result::ok).collect();
    let duration = start.elapsed().as_millis() as u64;

    Ok(QueryResult {
        schema,
        rows,
        execution_time_ms: duration,
        total_rows,
    })
}

pub fn export_file_list_query(
    conn: &Connection,
    file_paths: &[String],
    query: &str,
    output_path: &str,
    format: &str,
) -> Result<()> {
    let source_sql = build_source_sql(file_paths);

    let view_sql = format!(
        "CREATE OR REPLACE VIEW t AS SELECT * FROM read_parquet({}, filename=true)",
        source_sql
    );
    conn.execute(&view_sql, [])?;

    let (fmt_cmd, opts) = match format.to_uppercase().as_str() {
        "JSON" => ("JSON", "ARRAY true"),
        "PARQUET" => ("PARQUET", "COMPRESSION 'SNAPPY'"),
        _ => ("CSV", "HEADER true, DELIMITER ','"),
    };

    let clean_query = query.trim().trim_end_matches(";");

    let copy_sql = format!(
        "COPY ({}) TO '{}' (FORMAT {},{});",
        clean_query, output_path, fmt_cmd, opts
    );

    conn.execute(&copy_sql, [])?;
    Ok(())
}
