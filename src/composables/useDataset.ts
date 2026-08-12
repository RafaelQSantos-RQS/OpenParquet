// Composable replacing the old_code fileStore (.tmp/old_code/src/lib/stores/file.svelte.ts).
// One instance per call — no singletons. Reactive state via refs, pure functions over state.
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type {
  ColumnInfo,
  DatasetInfo,
  PageData,
  ParquetFileInfo,
  QueryResult,
  SortState,
  SourceDescriptor,
} from "../types";

const PAGE_SIZE = 50;

export function useDataset() {
  const schema = ref<ColumnInfo[]>([]);
  const rows = ref<PageData>([]);
  const totalRows = ref(0);
  const files = ref<ParquetFileInfo[]>([]);
  const currentPage = ref(0);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  const sort = ref<SortState>({ column: null, order: null });
  const isSqlMode = ref(false);
  const currentSqlQuery = ref("");
  const sqlExecutionTime = ref(0);
  const source = ref<SourceDescriptor | null>(null);

  const hasData = computed(() => schema.value.length > 0);
  const sourcePath = computed<string | null>(() => {
    const src = source.value;
    return src && src.type !== "list" ? src.path : null;
  });

  /** Opens a dataset (file, folder or list) and loads the first page if there is a schema. */
  async function openDataset(newSource: SourceDescriptor): Promise<void> {
    source.value = newSource;
    isLoading.value = true;
    error.value = null;
    resetData();

    try {
      const info = await invoke<DatasetInfo>("open_dataset", { source: newSource });
      schema.value = info.schema;
      totalRows.value = info.totalRows;
      files.value = info.files;

      if (schema.value.length > 0) {
        await loadPage(0);
      }
    } catch (e) {
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }

  /** Loads a page: via run_sql in SQL mode, via get_page (with sort) in dataset mode. */
  async function loadPage(page: number): Promise<void> {
    if (!source.value) return;
    isLoading.value = true;
    error.value = null;

    try {
      if (isSqlMode.value) {
        const result = await invoke<QueryResult>("run_sql", {
          source: source.value,
          query: currentSqlQuery.value,
          page,
          pageSize: PAGE_SIZE,
        });
        rows.value = result.rows;
      } else {
        rows.value = await invoke<PageData>("get_page", {
          source: source.value,
          page,
          pageSize: PAGE_SIZE,
          sortCol: sort.value.column,
          sortOrder: sort.value.order,
        });
      }
      currentPage.value = page;
    } catch (e) {
      error.value = String(e);
    } finally {
      isLoading.value = false;
    }
  }

  /** Runs an arbitrary SQL query and enters SQL mode. */
  async function runSql(query: string): Promise<void> {
    if (!source.value) return;
    isLoading.value = true;
    error.value = null;
    currentSqlQuery.value = query;

    try {
      const result = await invoke<QueryResult>("run_sql", {
        source: source.value,
        query,
        page: 0,
        pageSize: PAGE_SIZE,
      });
      schema.value = result.schema;
      rows.value = result.rows;
      totalRows.value = result.totalRows;
      sqlExecutionTime.value = result.executionTimeMs;
      isSqlMode.value = true;
      currentPage.value = 0;
      sort.value = { column: null, order: null };
    } catch (e) {
      error.value = `Query error: ${String(e)}`;
    } finally {
      isLoading.value = false;
    }
  }

  /** Exits SQL mode and reloads the original dataset. */
  async function exitSqlMode(): Promise<void> {
    isSqlMode.value = false;
    currentSqlQuery.value = "";
    if (source.value) {
      await openDataset(source.value);
    }
  }

  /** Resets the data state (keeps `source`). */
  function resetData(): void {
    schema.value = [];
    rows.value = [];
    totalRows.value = 0;
    files.value = [];
    currentPage.value = 0;
    sort.value = { column: null, order: null };
    isSqlMode.value = false;
    currentSqlQuery.value = "";
    sqlExecutionTime.value = 0;
  }

  /** Exports a query result to the chosen format. */
  async function exportDataset(query: string, outputPath: string, format: string): Promise<void> {
    if (!source.value) return;
    await invoke<void>("export_dataset", {
      source: source.value,
      query,
      outputPath,
      format,
    });
  }

  return {
    // estado
    schema,
    rows,
    totalRows,
    files,
    currentPage,
    pageSize: PAGE_SIZE,
    isLoading,
    error,
    sort,
    isSqlMode,
    currentSqlQuery,
    sqlExecutionTime,
    source,
    // derivados
    hasData,
    sourcePath,
    // actions
    openDataset,
    loadPage,
    runSql,
    exitSqlMode,
    resetData,
    exportDataset,
  };
}
