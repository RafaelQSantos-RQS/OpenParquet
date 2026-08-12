<script setup lang="ts">
// Controlled metadata dialog (opened from the titlebar button):
// path, stats (rows/files), file list (multi-file) and schema tags.
import { computed } from "vue";
import AppModal from "./AppModal.vue";
import type { ColumnInfo, ParquetFileInfo, SourceDescriptor } from "../types";

const props = defineProps<{
  open: boolean;
  source: SourceDescriptor | null;
  totalRows: number;
  schema: ColumnInfo[];
  files: ParquetFileInfo[];
}>();

const emit = defineEmits<{ close: [] }>();

const isMultiFileMode = computed(() => props.source?.type !== "file");
const formattedRows = computed(() => new Intl.NumberFormat("en-US").format(props.totalRows));

const sourceName = computed(() => {
  const src = props.source;
  if (!src) return "Parquet File";
  if (src.type === "list") return "File List";
  return src.path.split(/[/\\]/).pop() || "Parquet File";
});

const sourcePath = computed(() => {
  const src = props.source;
  return src && src.type !== "list" ? src.path : "";
});

const pathLabel = computed(() => {
  const src = props.source;
  if (src?.type === "dir") return "Folder Path";
  return "File Path";
});
</script>

<template>
  <AppModal v-if="open" :title="`Metadata — ${sourceName}`" @close="emit('close')">
    <div class="meta-content">
      <div class="info-grid">
        <div class="info-item full-width">
          <div class="meta-label">{{ pathLabel }}</div>
          <code class="path" :title="sourcePath">{{ sourcePath }}</code>
        </div>

        <div class="info-card">
          <div class="stat-value">{{ formattedRows }}</div>
          <div class="stat-label">Total Rows</div>
        </div>

        <div class="info-card">
          <div class="stat-value">{{ isMultiFileMode ? files.length : schema.length }}</div>
          <div class="stat-label">{{ isMultiFileMode ? "Files" : "Columns" }}</div>
        </div>
      </div>

      <div v-if="isMultiFileMode && files.length > 0" class="files-section">
        <div class="meta-label">Files ({{ files.length }})</div>
        <div class="files-list">
          <div v-for="file in files" :key="file.filePath" class="file-item" :title="file.filePath">
            <span class="file-icon"><i class="mdi mdi-file-outline" /></span>
            <span class="file-name">{{ file.fileName }}</span>
            <span class="file-rows">
              {{ new Intl.NumberFormat("en-US").format(file.rowCount) }} rows
            </span>
          </div>
        </div>
      </div>

      <div class="schema-section">
        <div class="meta-label">Schema</div>
        <div class="schema-tags">
          <span v-for="col in schema" :key="col.name" class="col-tag" :title="col.type">
            <span class="col-name">{{ col.name }}</span>
            <span class="col-type">{{ col.type }}</span>
          </span>
        </div>
      </div>
    </div>
  </AppModal>
</template>

<style scoped>
.meta-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.full-width {
  grid-column: span 2;
}

.meta-label {
  display: block;
  font-size: 0.7rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  margin-bottom: 0.5rem;
  font-weight: 600;
}

code.path {
  display: block;
  background: rgb(var(--v-theme-background));
  padding: 0.6rem;
  border-radius: 4px;
  font-family: "Consolas", "Monaco", monospace;
  font-size: 0.8rem;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  direction: rtl;
  text-align: left;
}

.info-card {
  background: rgb(var(--v-theme-background));
  padding: 0.75rem;
  border-radius: 4px;
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-left: 3px solid rgb(var(--v-theme-primary));
}

.stat-value {
  font-size: 1.4rem;
  font-weight: 700;
  color: rgb(var(--v-theme-on-surface));
  line-height: 1.2;
}

.stat-label {
  font-size: 0.8rem;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
}

.schema-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  max-height: 180px;
  overflow-y: auto;
  padding-right: 4px;
}

.col-tag {
  display: inline-flex;
  align-items: stretch;
  background-color: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 4px;
  font-size: 0.8rem;
  overflow: hidden;
}

.col-name {
  padding: 4px 8px;
  color: rgb(var(--v-theme-on-surface));
  font-weight: 500;
}

.col-type {
  background-color: rgba(var(--v-border-color), var(--v-border-opacity));
  padding: 4px 8px;
  color: rgb(var(--v-theme-primary));
  font-family: monospace;
  font-size: 0.75rem;
  border-left: 1px solid rgb(var(--v-theme-background));
}

.files-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  max-height: 200px;
  overflow-y: auto;
  padding-right: 4px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  background: rgb(var(--v-theme-background));
  padding: 0.5rem 0.75rem;
  border-radius: 4px;
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  font-size: 0.85rem;
}

.file-icon {
  color: rgb(var(--v-theme-primary));
  flex-shrink: 0;
}

.file-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: rgb(var(--v-theme-on-surface));
}

.file-rows {
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  font-size: 0.75rem;
  flex-shrink: 0;
}
</style>
