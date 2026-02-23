use crate::error::{AppError, AppResult};
use std::path::Path;

const FORBIDDEN_KEYWORDS: &[&str] = &["DROP", "DELETE", "TRUNCATE", "ALTER", "GRANT", "REVOKE"];

#[derive(Debug, Clone, PartialEq)]
pub enum SourceKind {
    File(String),
    Directory(String),
    FileList(Vec<String>),
}

fn validate_path_base(path: &str) -> AppResult<()> {
    if path.is_empty() {
        return Err(AppError::InvalidPath("Path cannot be empty".into()));
    }

    if !Path::new(path).is_absolute() {
        return Err(AppError::InvalidPath("Path must be absolute".into()));
    }

    if path.contains('\0') {
        return Err(AppError::InvalidPath("Path contains null character".into()));
    }

    Ok(())
}

pub fn validate_parquet_path(path: &str) -> AppResult<()> {
    validate_path_base(path)?;

    if !path.to_lowercase().ends_with(".parquet") {
        return Err(AppError::InvalidPath(
            "File must have .parquet extension".into(),
        ));
    }

    Ok(())
}

pub fn validate_parquet_directory(path: &str) -> AppResult<()> {
    validate_path_base(path)?;

    let p = Path::new(path);
    if !p.is_dir() {
        return Err(AppError::InvalidPath("Path must be a directory".into()));
    }

    Ok(())
}

pub fn validate_source_path(path: &str) -> AppResult<SourceKind> {
    validate_path_base(path)?;

    let p = Path::new(path);

    if p.is_dir() {
        return Ok(SourceKind::Directory(path.to_string()));
    }

    if p.is_file() && path.to_lowercase().ends_with(".parquet") {
        return Ok(SourceKind::File(path.to_string()));
    }

    Err(AppError::InvalidPath(
        "Path must be a .parquet file or a directory".into(),
    ))
}

pub fn validate_parquet_paths(paths: &[String]) -> AppResult<SourceKind> {
    if paths.is_empty() {
        return Err(AppError::InvalidPath("File list cannot be empty".into()));
    }

    for path in paths {
        validate_path_base(path)?;
        if !path.to_lowercase().ends_with(".parquet") {
            return Err(AppError::InvalidPath(format!(
                "File '{}' must have .parquet extension",
                path
            )));
        }
    }

    Ok(SourceKind::FileList(paths.to_vec()))
}

pub fn validate_sql_query(query: &str) -> AppResult<()> {
    let upper_query = query.to_uppercase();

    for keyword in FORBIDDEN_KEYWORDS {
        if upper_query.contains(keyword) {
            return Err(AppError::ForbiddenKeyword((*keyword).into()));
        }
    }

    Ok(())
}
