<script setup lang="ts">
import { onMounted } from "vue";
import Titlebar from "./Titlebar.vue";
import { useDragDrop } from "../composables/useDragDrop";

const props = defineProps<{
  hasData: boolean;
  titleText: string;
  isSqlMode: boolean;
  isLoading: boolean;
  sqlTime: number;
}>();

const emit = defineEmits<{
  "open-file": [];
  "open-files": [];
  "open-directory": [];
  "drop-paths": [paths: string[]];
  export: [];
  sql: [];
  metadata: [];
}>();

const { isDragging, onDrop } = useDragDrop();

onMounted(() => {
  void onDrop((paths) => emit("drop-paths", paths));
});
</script>

<template>
  <div class="layout">
    <Titlebar
      @open-file="emit('open-file')"
      @open-files="emit('open-files')"
      @open-directory="emit('open-directory')"
      :has-data="props.hasData"
      :title-text="props.titleText"
      :is-sql-mode="props.isSqlMode"
      :is-loading="props.isLoading"
      :sql-time="props.sqlTime"
      @export="emit('export')"
      @sql="emit('sql')"
      @metadata="emit('metadata')"
    />
    <main class="layout-content">
      <slot />
      <div v-if="isDragging" class="drop-overlay">
        <div class="drop-message">
          <i class="mdi mdi-upload" />
          <span>Drop Parquet files or folders here</span>
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.layout {
  display: flex;
  flex-direction: column;
  height: 100dvh;
}

.layout-content {
  position: relative;
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  background-color: rgb(var(--v-theme-background));
}

.drop-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(var(--v-theme-background), 0.85);
  z-index: 9999;
  display: flex;
  justify-content: center;
  align-items: center;
  backdrop-filter: blur(4px);
  pointer-events: none;
}

.drop-message {
  border: 3px dashed rgb(var(--v-theme-primary));
  border-radius: 12px;
  padding: 3rem 5rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  color: rgb(var(--v-theme-primary));
  background-color: rgba(var(--v-theme-primary), 0.1);
}

.drop-message i {
  font-size: 4rem;
  animation: bounce 1.5s infinite;
}

.drop-message span {
  font-size: 1.5rem;
  font-weight: 600;
}

@keyframes bounce {
  0%,
  100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}
</style>
