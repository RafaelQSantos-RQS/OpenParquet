// src-tauri/src/db_logic.rs
use crate::models::{ColumnInfo, DatasetInfo, ParquetFileInfo, QueryResult, SourceDescriptor};
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
        ValueRef::UHugeInt(u) => u.to_string(),
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

/// Escapes an SQL string literal (doubles single quotes). DuckDB DDL doesn't accept
/// prepared parameters, so the path goes in via an escaped literal.
fn quote_literal(s: &str) -> String {
    format!("'{}'", s.replace('\'', "''"))
}

/// SQL identifier wrapped in double quotes with escaping.
fn quote_ident(s: &str) -> String {
    format!("\"{}\"", s.replace('"', "\"\""))
}

/// The SQL source (FROM clause) for each SourceDescriptor type.
fn source_from_sql(source: &SourceDescriptor) -> String {
    match source {
        SourceDescriptor::File { path } => format!("read_parquet({})", quote_literal(path)),
        SourceDescriptor::Dir { path } => {
            let glob = format!("{}/**/*.parquet", path);
            format!("read_parquet({}, filename=true)", quote_literal(&glob))
        }
        SourceDescriptor::List { paths } => {
            let quoted: Vec<String> = paths.iter().map(|p| quote_literal(p)).collect();
            format!("read_parquet([{}], filename=true)", quoted.join(", "))
        }
    }
}

fn read_schema(conn: &Connection, from: &str) -> Result<Vec<ColumnInfo>> {
    let sql = format!("DESCRIBE SELECT * FROM {}", from);
    let mut stmt = conn.prepare(&sql)?;

    let columns_iter = stmt.query_map([], |row| {
        Ok(ColumnInfo {
            name: row.get(0)?,
            type_: row.get(1)?,
        })
    })?;

    Ok(columns_iter.filter_map(Result::ok).collect())
}

pub fn get_schema(conn: &Connection, source: &SourceDescriptor) -> Result<Vec<ColumnInfo>> {
    read_schema(conn, &source_from_sql(source))
}

pub fn list_files(conn: &Connection, source: &SourceDescriptor) -> Result<Vec<ParquetFileInfo>> {
    let from = source_from_sql(source);
    let sql = format!(
        "SELECT filename, COUNT(*) as row_count FROM {} GROUP BY filename ORDER BY filename",
        from
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

pub fn get_dataset_info(conn: &Connection, source: &SourceDescriptor) -> Result<DatasetInfo> {
    let schema = get_schema(conn, source)?;
    let files = list_files(conn, source)?;
    let total_rows: i64 = files.iter().map(|f| f.row_count).sum();

    Ok(DatasetInfo {
        schema,
        total_rows,
        files,
    })
}

pub fn get_page(
    conn: &Connection,
    source: &SourceDescriptor,
    col_names: &[String],
    limit: usize,
    offset: usize,
    sort_col: Option<&str>,
    sort_order: Option<&str>,
) -> Result<Vec<HashMap<String, String>>> {
    let from = source_from_sql(source);

    let select_casts = col_names
        .iter()
        .map(|name| format!("{}::VARCHAR", quote_ident(name)))
        .collect::<Vec<String>>()
        .join(", ");

    let order_clause = match (sort_col, sort_order) {
        (Some(col), Some(order)) => {
            let direction = if order.eq_ignore_ascii_case("DESC") {
                "DESC"
            } else {
                "ASC"
            };
            format!("ORDER BY {} {}", quote_ident(col), direction)
        }
        _ => String::new(),
    };

    let sql = format!(
        "SELECT {} FROM {} {} LIMIT {} OFFSET {}",
        select_casts, from, order_clause, limit, offset
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

pub fn exec_query(
    conn: &Connection,
    source: &SourceDescriptor,
    user_query: &str,
    limit: usize,
    offset: usize,
) -> Result<QueryResult> {
    let start = Instant::now();
    let from = source_from_sql(source);

    let view_sql = format!("CREATE OR REPLACE VIEW t AS SELECT * FROM {}", from);
    conn.execute(&view_sql, [])?;

    let clean_query = user_query.trim().trim_end_matches(';');

    let count_sql = format!("SELECT COUNT(*) FROM ({})", clean_query);
    let total_rows: i64 = conn.query_row(&count_sql, [], |row| row.get(0))?;

    let schema = read_schema(conn, &format!("({})", clean_query))?;

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

pub fn export_query(
    conn: &Connection,
    source: &SourceDescriptor,
    query: &str,
    output_path: &str,
    format: &str,
) -> Result<()> {
    let from = source_from_sql(source);

    let view_sql = format!("CREATE OR REPLACE VIEW t AS SELECT * FROM {}", from);
    conn.execute(&view_sql, [])?;

    let (fmt_cmd, opts) = match format.to_uppercase().as_str() {
        "JSON" => ("JSON", "ARRAY true"),
        "PARQUET" => ("PARQUET", "COMPRESSION 'SNAPPY'"),
        _ => ("CSV", "HEADER true, DELIMITER ','"),
    };

    let clean_query = query.trim().trim_end_matches(';');

    let copy_sql = format!(
        "COPY ({}) TO {} (FORMAT {},{});",
        clean_query,
        quote_literal(output_path),
        fmt_cmd,
        opts
    );

    conn.execute(&copy_sql, [])?;
    Ok(())
}
