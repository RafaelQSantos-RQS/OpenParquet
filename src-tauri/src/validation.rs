use crate::error::{AppError, AppResult};
use crate::models::SourceDescriptor;
use std::path::Path;

const FORBIDDEN_KEYWORDS: &[&str] = &["DROP", "DELETE", "TRUNCATE", "ALTER", "GRANT", "REVOKE"];

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

fn validate_parquet_file(path: &str) -> AppResult<()> {
    validate_path_base(path)?;

    if !path.to_lowercase().ends_with(".parquet") {
        return Err(AppError::InvalidPath(
            "File must have .parquet extension".into(),
        ));
    }

    Ok(())
}

/// Validates the source at the trust boundary (IPC).
pub fn validate_source(source: &SourceDescriptor) -> AppResult<()> {
    match source {
        SourceDescriptor::File { path } => validate_parquet_file(path),
        SourceDescriptor::Dir { path } => {
            validate_path_base(path)?;

            if !Path::new(path).is_dir() {
                return Err(AppError::InvalidPath("Path must be a directory".into()));
            }

            Ok(())
        }
        SourceDescriptor::List { paths } => {
            if paths.is_empty() {
                return Err(AppError::InvalidPath("File list cannot be empty".into()));
            }

            for path in paths {
                validate_parquet_file(path)?;
            }

            Ok(())
        }
    }
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
