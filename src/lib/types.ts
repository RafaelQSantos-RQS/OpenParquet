export interface ColumnInfo {
	name: string;
	type: string;
}

export type DataRow = Record<string, string>;

export interface FileMetadata {
	file_path: string;
	total_rows: number;
	schema: ColumnInfo[];
}

export interface ParquetFileInfo {
	file_name: string;
	file_path: string;
	row_count: number;
}

export interface MultiFileMetadata {
	directory_path: string;
	files: ParquetFileInfo[];
	total_rows: number;
	schema: ColumnInfo[];
}

export interface FileListMetadata {
	files: ParquetFileInfo[];
	total_rows: number;
	schema: ColumnInfo[];
}

export interface QueryResult {
	schema: ColumnInfo[];
	rows: DataRow[];
	execution_time_ms: number;
	total_rows: number;
}

export type SortOrder = 'ASC' | 'DESC';

export interface SortState {
	column: string | null;
	order: SortOrder | null;
}

export type Theme = 'dark' | 'light';

export type ExportFormat = 'CSV' | 'JSON' | 'PARQUET';

export type ExportScope = 'all' | 'query';

export type DataSource = 'file' | 'directory' | 'fileList';
