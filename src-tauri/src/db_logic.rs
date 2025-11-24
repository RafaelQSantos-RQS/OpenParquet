
use crate::models::{ColumnInfo, PageData, QueryResult};
use duckdb::types::ValueRef;
use duckdb::{Connection, Result};
use std::collections::HashMap;
use std::time::Instant;
use chrono::NaiveDate;


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
        ValueRef::Date32(days) => {
            match NaiveDate::from_ymd_opt(1970, 1, 1) {
                Some(epoch) => match epoch.checked_add_signed(chrono::Duration::days(days as i64)) {
                    Some(d) => d.format("%Y-%m-%d").to_string(),
                    None => days.to_string()
                },
                None => days.to_string()
            }
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
        let direction = if order.to_uppercase() == "DESC" { "DESC" } else { "ASC" };
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


pub fn exec_custom_query(
    conn: &Connection, 
    file_path: &str, 
    user_query: &str,
    limit: usize,  
    offset: usize  
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
    for col in schema_iter {
        if let Ok(c) = col { schema.push(c); }
    }

    
    let paged_sql = format!("SELECT * FROM ({}) LIMIT {} OFFSET {}", clean_query, limit, offset);
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
    let duration = start.elapsed().as_millis();

    Ok(QueryResult {
        schema,
        rows,
        execution_time_ms: duration,
        total_rows, 
    })
}