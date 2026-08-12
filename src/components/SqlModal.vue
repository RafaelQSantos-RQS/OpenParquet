<script setup lang="ts">
// SQL modal replicating the old_code SqlModal: view t hint + textarea + actions.
import { ref } from "vue";
import AppModal from "./AppModal.vue";
import { useUiStore } from "../stores/ui";

const ui = useUiStore();

const emit = defineEmits<{
  run: [query: string];
}>();

const query = ref("SELECT * FROM t LIMIT 10");

function run(): void {
  if (!query.value.trim()) return;
  emit("run", query.value);
}
</script>

<template>
  <AppModal v-if="ui.showSql" title="Run SQL Query" @close="ui.closeSql">
    <div class="sql-container">
      <p class="hint">
        The current table is available as view <code>t</code>.
      </p>

      <textarea
        v-model="query"
        placeholder="e.g. SELECT count(*) FROM t WHERE value > 100"
        spellcheck="false"
      ></textarea>

      <div class="actions">
        <button class="btn-secondary" type="button" @click="ui.closeSql">Cancel</button>
        <button class="btn-primary" type="button" @click="run">
          <i class="mdi mdi-play" />
          Run Query
        </button>
      </div>
    </div>
  </AppModal>
</template>

<style scoped>
.sql-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  height: 100%;
}

.hint {
  font-size: 0.9rem;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  margin: 0;
}

code {
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  padding: 2px 6px;
  border-radius: 4px;
  color: rgb(var(--v-theme-primary));
  font-family: monospace;
  font-weight: bold;
}

textarea {
  background-color: rgb(var(--v-theme-background));
  color: rgb(var(--v-theme-on-surface));
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 6px;
  padding: 1rem;
  font-family: "Consolas", "Monaco", "Courier New", monospace;
  font-size: 0.95rem;
  min-height: 200px;
  flex: 1;
  resize: vertical;
  outline: none;
  line-height: 1.5;
}

textarea:focus {
  border-color: rgb(var(--v-theme-primary));
  box-shadow: 0 0 0 1px rgb(var(--v-theme-primary));
}

.actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.8rem;
  margin-top: auto;
  padding-top: 0.5rem;
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
  transition: background 0.2s;
}

.btn-secondary:hover {
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
}

.btn-primary {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background-color: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-on-primary));
  border: none;
  padding: 0.6rem 1.2rem;
  border-radius: 6px;
  font-weight: 600;
  font-family: inherit;
  font-size: 0.9rem;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background-color: rgb(var(--v-theme-secondary));
}
</style>
