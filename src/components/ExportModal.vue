<script setup lang="ts">
// Export modal replicating the old_code ExportModal:
// format-grid (CSV/JSON/Parquet) + scope (Full Table / Current View Result).
import { ref, watch } from "vue";
import { save } from "@tauri-apps/plugin-dialog";
import AppModal from "./AppModal.vue";
import type { useDataset } from "../composables/useDataset";
import { useUiStore } from "../stores/ui";

const props = defineProps<{ dataset: ReturnType<typeof useDataset> }>();
const { isSqlMode, currentSqlQuery, sort, exportDataset } = props.dataset;

const ui = useUiStore();

const FORMATS = [
  { value: "CSV", label: "CSV" },
  { value: "JSON", label: "JSON" },
  { value: "PARQUET", label: "Parquet" },
] as const;

const selectedFormat = ref<"CSV" | "JSON" | "PARQUET">("CSV");
const selectedScope = ref<"all" | "query">("all");

// On open, pre-select the scope according to the current mode.
watch(
  () => ui.showExport,
  (visible) => {
    if (visible) selectedScope.value = isSqlMode.value ? "query" : "all";
  },
);

const snack = ref(false);
const snackText = ref("");
const snackColor = ref<"success" | "error">("success");

function notify(text: string, color: "success" | "error"): void {
  snackText.value = text;
  snackColor.value = color;
  snack.value = true;
}

function buildQuery(): string {
  let query = "SELECT * FROM t";
  if (selectedScope.value === "query" && currentSqlQuery.value) {
    return currentSqlQuery.value;
  }
  if (!isSqlMode.value && sort.value.column && sort.value.order) {
    query += ` ORDER BY "${sort.value.column}" ${sort.value.order}`;
  }
  return query;
}

async function handleExport(): Promise<void> {
  const ext = selectedFormat.value.toLowerCase();
  try {
    const savePath = await save({
      title: "Save Exported File",
      defaultPath: `export.${ext}`,
      filters: [{ name: selectedFormat.value, extensions: [ext] }],
    });
    if (!savePath) return;

    ui.setExporting(true);
    await exportDataset(buildQuery(), savePath, selectedFormat.value.toLowerCase());
    ui.closeExport();
    notify(`Data exported successfully to ${savePath}!`, "success");
  } catch (e) {
    notify(String(e), "error");
  } finally {
    ui.setExporting(false);
  }
}
</script>

<template>
  <AppModal v-if="ui.showExport" title="Export Data" @close="ui.closeExport">
    <div class="export-options">
      <div>
        <div class="section-title">File Format</div>
        <div class="format-grid">
          <button
            v-for="fmt in FORMATS"
            :key="fmt.value"
            type="button"
            class="format-btn"
            :class="{ selected: selectedFormat === fmt.value }"
            @click="selectedFormat = fmt.value"
          >
            <i class="mdi mdi-file-outline" />
            <span class="format-label">{{ fmt.label }}</span>
          </button>
        </div>
      </div>

      <div>
        <div class="section-title">Data to Export</div>
        <div class="radio-group">
          <label class="radio-item">
            <input
              v-model="selectedScope"
              type="radio"
              value="all"
              :disabled="isSqlMode"
            />
            <div>
              <strong>Full Table</strong>
              <div class="radio-desc">All data from the original file.</div>
            </div>
          </label>

          <label class="radio-item">
            <input
              v-model="selectedScope"
              type="radio"
              value="query"
              :disabled="!isSqlMode"
            />
            <div>
              <strong>Current View Result</strong>
              <div class="radio-desc">
                <template v-if="isSqlMode">Only the data resulting from your SQL query.</template>
                <template v-else>(Available only in SQL Mode)</template>
              </div>
            </div>
          </label>
        </div>
      </div>

      <div class="actions">
        <button class="btn-secondary" type="button" :disabled="ui.isExporting" @click="ui.closeExport">
          Cancel
        </button>
        <button class="btn-primary" type="button" :disabled="ui.isExporting" @click="handleExport">
          {{ ui.isExporting ? "Exporting..." : "Save File..." }}
        </button>
      </div>
    </div>

    <v-snackbar v-model="snack" :color="snackColor" :timeout="4000">
      {{ snackText }}
    </v-snackbar>
  </AppModal>
</template>

<style scoped>
.export-options {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  min-width: 400px;
}

.section-title {
  font-size: 0.9rem;
  font-weight: 600;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  margin-bottom: 0.5rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.format-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

.format-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 1rem;
  background-color: rgb(var(--v-theme-background));
  border: 2px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  color: rgb(var(--v-theme-on-surface));
  font-family: inherit;
}

.format-btn:hover {
  border-color: rgb(var(--v-theme-primary));
  background-color: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
}

.format-btn.selected {
  border-color: rgb(var(--v-theme-primary));
  background-color: rgb(var(--v-theme-primary), 0.1);
  color: rgb(var(--v-theme-primary));
}

.format-btn i {
  font-size: 1.5rem;
}

.format-label {
  font-weight: 600;
  font-size: 0.9rem;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 0.8rem;
}

.radio-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  padding: 0.5rem;
  border-radius: 6px;
}

.radio-item:hover {
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
}

.radio-item input {
  accent-color: rgb(var(--v-theme-primary));
  width: 16px;
  height: 16px;
}

.radio-desc {
  font-size: 0.8rem;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.8rem;
  margin-top: 1rem;
  border-top: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  padding-top: 1rem;
}

.btn-secondary {
  background: transparent;
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  color: rgb(var(--v-theme-on-surface));
  padding: 0.6rem 1.2rem;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  font-family: inherit;
  font-size: 0.9rem;
}

.btn-secondary:hover:not(:disabled) {
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
}

.btn-primary {
  background-color: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-on-primary));
  border: none;
  padding: 0.6rem 1.2rem;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  font-family: inherit;
  font-size: 0.9rem;
}

.btn-primary:hover:not(:disabled) {
  background-color: rgb(var(--v-theme-secondary));
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
</style>
