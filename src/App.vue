<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import AppLayout from "./components/AppLayout.vue";
import DataTable from "./components/DataTable.vue";
import MetadataPanel from "./components/MetadataPanel.vue";
import SqlModal from "./components/SqlModal.vue";
import ExportModal from "./components/ExportModal.vue";
import AboutModal from "./components/AboutModal.vue";
import RecentFiles from "./components/RecentFiles.vue";
import { useDataset } from "./composables/useDataset";
import { useUiStore } from "./stores/ui";
import { useRecentsStore } from "./stores/recents";
import { APP_INFO } from "./constants";
import type { RecentEntry } from "./stores/recents";
import type { SourceDescriptor } from "./types";

const ui = useUiStore();
const recents = useRecentsStore();
const dataset = useDataset();

const showMetadata = ref(false);

const {
  schema,
  rows,
  totalRows,
  files,
  source,
  sort,
  isLoading,
  error,
  currentPage,
  pageSize,
  isSqlMode,
  sqlExecutionTime,
  hasData,
} = dataset;

function sourceFromPaths(paths: string[]): SourceDescriptor {
  if (paths.length === 1) {
    const p = paths[0];
    // extension decides file vs dir; list for multiple
    return /\.parquet$/i.test(p) ? { type: "file", path: p } : { type: "dir", path: p };
  }
  return { type: "list", paths };
}

async function openSource(source: SourceDescriptor): Promise<void> {
  await dataset.openDataset(source);
  await recents.add({
    type: source.type,
    paths: source.type === "list" ? source.paths : [source.path],
  });
}

async function openFilesDialog(multiple: boolean, directory: boolean): Promise<void> {
  const result = await open({
    multiple,
    directory,
    filters: directory ? undefined : [{ name: "Parquet", extensions: ["parquet"] }],
  });
  if (!result) return;
  const paths = Array.isArray(result) ? result : [result];
  if (paths.length > 0) await openSource(sourceFromPaths(paths));
}

function onOpenFile(): void {
  void openFilesDialog(false, false);
}
function onOpenFiles(): void {
  void openFilesDialog(true, false);
}
function onOpenDirectory(): void {
  void openFilesDialog(false, true);
}
function onDropPaths(paths: string[]): void {
  void openSource(sourceFromPaths(paths));
}
function onOpenRecent(entry: RecentEntry): void {
  const src: SourceDescriptor =
    entry.type === "list"
      ? { type: "list", paths: entry.paths }
      : { type: entry.type, path: entry.paths[0] };
  void openSource(src);
}

interface TableOptions {
  page: number; // 1-based
  sortBy: { key: string; order: "asc" | "desc" }[];
}

/** Bridge v-data-table-server → dataset: applies sort and reloads the requested page. */
function onTableOptions({ page, sortBy }: TableOptions): void {
  const s = sortBy[0];
  const sortChanged = s
    ? !(sort.value.column === s.key && sort.value.order === (s.order === "desc" ? "DESC" : "ASC"))
    : sort.value.column !== null;

  if (sortChanged) {
    sort.value = s
      ? { column: s.key, order: s.order === "desc" ? "DESC" : "ASC" }
      : { column: null, order: null };
  }

  const targetPage = page - 1;
  if (targetPage !== currentPage.value) {
    void dataset.loadPage(targetPage);
  } else if (sortChanged) {
    void dataset.loadPage(currentPage.value);
  }
}

const sourceName = computed(() => {
  const src = source.value;
  if (!src) return "";
  if (src.type === "list") return "File List";
  return src.path.split(/[/\\]/).pop() || "Parquet File";
});

const formattedTotalRows = computed(() =>
  new Intl.NumberFormat("en-US").format(totalRows.value),
);

// Titlebar title: file summary while data is loaded, app name otherwise.
const titleText = computed(() =>
  hasData.value ? `${sourceName.value} · ${formattedTotalRows.value} rows` : APP_INFO.name,
);

function onRunSql(query: string): void {
  ui.closeSql();
  void dataset.runSql(query);
}

onMounted(() => {
  void recents.load();
});
</script>

<template>
  <v-app>
    <AppLayout
      @open-file="onOpenFile"
      @open-files="onOpenFiles"
      @open-directory="onOpenDirectory"
      @drop-paths="onDropPaths"
      :has-data="hasData"
      :title-text="titleText"
      :is-sql-mode="isSqlMode"
      :is-loading="isLoading"
      :sql-time="sqlExecutionTime"
      @export="ui.openExport()"
      @sql="isSqlMode ? dataset.exitSqlMode() : ui.openSql()"
      @metadata="showMetadata = true"
    >
      <div class="content">
        <!-- Welcome: no data and no error -->
        <div v-if="!hasData && !error" class="empty-state-container">
          <img src="/welcome.svg" alt="Welcome to OpenParquet" class="welcome-image" />
          <h2 class="welcome-title">{{ APP_INFO.name }}</h2>
          <p class="welcome-subtitle">Drag files/folders or click below to get started.</p>

          <div class="open-buttons">
            <button class="btn-primary large-btn" type="button" :disabled="isLoading" @click="onOpenFile">
              {{ isLoading ? "Loading..." : "Open File" }}
            </button>
            <button class="btn-secondary large-btn" type="button" :disabled="isLoading" @click="onOpenFiles">
              Multiple Files
            </button>
            <button class="btn-secondary large-btn" type="button" :disabled="isLoading" @click="onOpenDirectory">
              Open Folder
            </button>
          </div>

          <div v-if="recents.entries.length > 0" class="recent-files">
            <h3>Recent Files</h3>
            <RecentFiles @open="onOpenRecent" />
          </div>
        </div>

        <!-- Error: banner + try another file -->
        <div v-if="error" class="actions-bar">
          <div class="error-banner">
            <strong>Error:</strong> {{ error }}
          </div>
          <button class="btn-primary" type="button" @click="onOpenFile">Try Another File</button>
        </div>

        <!-- Data: table only (actions live in the titlebar) -->
        <template v-if="hasData">
          <div class="flex-table-container">
            <DataTable
              :schema="schema"
              :rows="rows"
              :page="currentPage"
              :page-size="pageSize"
              :total-rows="totalRows"
              :sort="sort"
              :loading="isLoading"
              @options="onTableOptions"
            />
          </div>
        </template>
      </div>

      <MetadataPanel
        :open="showMetadata"
        :source="source"
        :total-rows="totalRows"
        :schema="schema"
        :files="files"
        @close="showMetadata = false"
      />
      <SqlModal @run="onRunSql" />
      <ExportModal :dataset="dataset" />
      <AboutModal />
    </AppLayout>
  </v-app>
</template>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
}

/* Main layout: replicating the old_code container/content */
.content {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  padding: 1rem;
  gap: 0.5rem;
  overflow: hidden;
  position: relative;
  box-sizing: border-box;
}

/* Error area */
.actions-bar {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 1rem;
}

.error-banner {
  background-color: rgba(189, 44, 0, 0.15);
  color: #ff8080;
  padding: 0.6rem 1rem;
  border-radius: 6px;
  font-size: 0.9rem;
  border: 1px solid rgb(var(--v-theme-error));
}

/* Table container (internal scroll) */
.flex-table-container {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 6px;
  background-color: rgb(var(--v-theme-surface));
  overflow: hidden;
}

/* General buttons (old_code pattern) */
.btn-primary {
  background-color: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-on-primary));
  border: 1px solid rgba(255, 255, 255, 0.1);
  padding: 0.6rem 1.2rem;
  border-radius: 6px;
  font-weight: 600;
  font-size: 0.9rem;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s;
  font-family: inherit;
}

.btn-primary:hover:not(:disabled) {
  background-color: rgb(var(--v-theme-secondary));
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  filter: grayscale(1);
}

.btn-secondary {
  background-color: rgb(var(--v-theme-surface));
  color: rgb(var(--v-theme-on-surface));
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  padding: 0.6rem 1.2rem;
  border-radius: 6px;
  font-weight: 600;
  font-size: 0.9rem;
  cursor: pointer;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  transition: all 0.2s;
  font-family: inherit;
}

.btn-secondary:hover:not(:disabled) {
  background-color: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  border-color: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-primary));
}

.btn-secondary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

/* Welcome screen */
.empty-state-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  text-align: center;
  padding-bottom: 4rem;
  overflow-y: auto;
}

.welcome-image {
  height: 280px;
  width: auto;
  margin-bottom: 2rem;
  filter: drop-shadow(0 10px 15px rgba(0, 0, 0, 0.15));
}

.welcome-title {
  font-size: 2.2rem;
  font-weight: 700;
  margin: 0 0 0.5rem;
  background: linear-gradient(
    to right,
    rgb(var(--v-theme-primary)),
    rgb(var(--v-theme-info))
  );
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  display: inline-block;
}

.welcome-subtitle {
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  margin-bottom: 2.5rem;
  font-size: 1.1rem;
}

.open-buttons {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  justify-content: center;
}

.large-btn {
  padding: 0.9rem 2.2rem;
  font-size: 1.05rem;
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.3);
}

.large-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.4);
}

/* Recentes na welcome screen */
.recent-files {
  margin-top: 2rem;
  width: 100%;
  max-width: 500px;
}

.recent-files h3 {
  font-size: 0.9rem;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  margin-bottom: 0.8rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  text-align: center;
}
</style>
