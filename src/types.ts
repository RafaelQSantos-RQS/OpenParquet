// Serialization contract with the backend (src-tauri/src/models.rs + commands.rs).
// Args and returns in camelCase: commands have #[tauri::command(rename_all = "camelCase")]
// and output structs have #[serde(rename_all = "camelCase")] (models.rs).

/** Data source: single file, folder or file list. */
export type SourceDescriptor =
  | { type: "file"; path: string }
  | { type: "dir"; path: string }
  | { type: "list"; paths: string[] };

export interface ColumnInfo {
  name: string;
  type: string;
}

export interface ParquetFileInfo {
  fileName: string;
  filePath: string;
  rowCount: number;
}

export interface DatasetInfo {
  schema: ColumnInfo[];
  totalRows: number;
  files: ParquetFileInfo[];
}

/** Row = column map -> value (values serialized as string). */
export type PageData = Record<string, string>[];

export interface QueryResult {
  schema: ColumnInfo[];
  rows: Record<string, string>[];
  executionTimeMs: number;
  totalRows: number;
}

export interface SortState {
  column: string | null;
  order: "ASC" | "DESC" | null;
}
