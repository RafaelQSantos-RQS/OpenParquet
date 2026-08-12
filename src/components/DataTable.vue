<script setup lang="ts">
// v-data-table-server wrapper (native server-side pagination/sort).
// Old_code look preserved via slots: # column, header with name + type + sort icons.
import { computed } from "vue";
import type { ColumnInfo, PageData, SortState } from "../types";

const props = defineProps<{
  schema: ColumnInfo[];
  rows: PageData;
  page: number; // 0-based
  pageSize: number;
  totalRows: number;
  sort: SortState;
  loading: boolean;
}>();

interface TableOptions {
  page: number; // 1-based
  sortBy: { key: string; order: "asc" | "desc" }[];
}

const emit = defineEmits<{
  options: [payload: TableOptions];
}>();

// # column (non-sortable) + one per schema column (type kept for the header slot)
const dataColumns = computed(() =>
  props.schema.map((c) => ({
    key: c.name,
    title: c.name,
    sortable: true,
    align: "start" as const,
    type: c.type,
  })),
);

const headers = computed(() => [
  { key: "__row", title: "#", sortable: false, align: "center" as const, width: 56 },
  ...dataColumns.value,
]);

// Stable key per row (Vuetify uses item-value as the v-for key)
const tableRows = computed(() =>
  props.rows.map((r, i) => ({ ...r, __row: props.page * props.pageSize + i + 1 })),
);

const sortBy = computed<{ key: string; order: "asc" | "desc" }[]>(() =>
  props.sort.column
    ? [{ key: props.sort.column, order: props.sort.order === "DESC" ? "desc" : "asc" }]
    : [],
);

function onUpdateOptions(options: TableOptions): void {
  emit("options", options);
}
</script>

<template>
  <v-data-table-server
    class="op-data-table"
    :headers="headers"
    :items="tableRows"
    :items-length="totalRows"
    :items-per-page="pageSize"
    :items-per-page-options="[pageSize]"
    :page="page + 1"
    :sort-by="sortBy"
    :loading="loading"
    hover
    density="compact"
    item-value="__row"
    fixed-header
    no-data-text="No data"
    @update:options="onUpdateOptions"
  >
    <template
      v-for="col in dataColumns"
      :key="col.key"
      #[`header.${col.key}`]="{ column, isSorted }"
    >
      <div class="th-content">
        <span>{{ col.title }}</span>
        <span v-if="isSorted(column)" class="sort-icon active">
          <i :class="sort.order === 'DESC' ? 'mdi mdi-arrow-down' : 'mdi mdi-arrow-up'" />
        </span>
        <span v-else class="sort-icon ghost">
          <i class="mdi mdi-arrow-up-down" />
        </span>
      </div>
      <div class="col-type">({{ col.type }})</div>
    </template>
    <template #item.__row="{ item }">
      {{ item.__row }}
    </template>
  </v-data-table-server>
</template>

<style scoped>
.op-data-table {
  flex: 1 1 auto;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

:deep(.v-table__wrapper) {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
}

/* Padding próprio (vence a regra de density do Vuetify via especificidade da classe) */
:deep(.v-data-table__td) {
  padding: 0.25rem 0.6rem !important;
  white-space: nowrap;
}

:deep(.v-data-table__th) {
  padding: 0.7rem 0.6rem !important;
}

/* Zebra (old_code pattern) */
:deep(.v-data-table tbody tr:nth-child(even)) {
  background-color: rgba(var(--v-theme-on-surface), 0.02);
}

/* Footer: only "1-50 of 1,234" + arrows */
:deep(.v-data-table-footer__items-per-page) {
  display: none;
}

:deep(.v-data-table-footer) {
  flex-shrink: 0;
}

.th-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.col-type {
  font-size: 0.65rem;
  font-weight: normal;
  opacity: 0.6;
  margin-top: 2px;
  text-transform: none;
}

.sort-icon {
  display: flex;
  align-items: center;
}

.sort-icon.active {
  color: rgb(var(--v-theme-primary));
}

.sort-icon.ghost {
  opacity: 0;
  transition: opacity 0.2s;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
}

:deep(.v-data-table th:hover) .sort-icon.ghost {
  opacity: 0.4;
}
</style>
