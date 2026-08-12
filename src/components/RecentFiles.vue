<script setup lang="ts">
import { onMounted } from "vue";
import { useRecentsStore, type RecentEntry } from "../stores/recents";

const emit = defineEmits<{ open: [entry: RecentEntry] }>();

const recents = useRecentsStore();

const typeIcon = {
  file: "mdi mdi-file-outline",
  dir: "mdi mdi-folder-outline",
  list: "mdi mdi-format-list-bulleted",
} as const;

onMounted(() => {
  recents.load();
});

function open(entry: RecentEntry): void {
  emit("open", entry);
}
</script>

<template>
  <div class="recent-files">
    <p v-if="recents.entries.length === 0" class="empty">No recent files</p>
    <template v-else>
      <ul class="list">
        <li v-for="entry in recents.entries" :key="`${entry.type}:${entry.paths.join(',')}`">
          <button type="button" class="btn-recent" @click="open(entry)">
            <span class="icon-file"><i :class="typeIcon[entry.type]" /></span>
            <span class="file-path" :title="recents.label(entry)">{{ recents.label(entry) }}</span>
          </button>
        </li>
      </ul>
      <button class="clear" type="button" @click="recents.clear()">Clear all</button>
    </template>
  </div>
</template>

<style scoped>
.recent-files {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.empty {
  margin: 0;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  font-style: italic;
  text-align: center;
}

.list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.btn-recent {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  width: 100%;
  padding: 0.8rem;
  background-color: rgb(var(--v-theme-surface));
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 6px;
  color: rgb(var(--v-theme-on-surface));
  cursor: pointer;
  transition: all 0.2s;
  text-align: left;
  font-family: inherit;
  font-size: 0.9rem;
}

.btn-recent:hover {
  border-color: rgb(var(--v-theme-primary));
  background-color: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  transform: translateX(4px);
}

.btn-recent .icon-file {
  color: rgb(var(--v-theme-primary));
  display: flex;
}

.file-path {
  font-size: 0.85rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  direction: rtl;
  text-align: left;
  flex: 1;
}

.clear {
  align-self: center;
  background: none;
  border: none;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  cursor: pointer;
  padding: 0.25rem 0.5rem;
  font-size: 0.8rem;
  font-family: inherit;
}

.clear:hover {
  color: rgb(var(--v-theme-primary));
}
</style>
