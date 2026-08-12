<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { onBeforeUnmount, onMounted, ref } from "vue";
import { useUiStore } from "../stores/ui";

const emit = defineEmits<{
  "open-file": [];
  "open-files": [];
  "open-directory": [];
  export: [];
  sql: [];
  metadata: [];
}>();

const props = defineProps<{
  hasData: boolean;
  titleText: string;
  isSqlMode: boolean;
  isLoading: boolean;
  sqlTime: number;
}>();

const ui = useUiStore();
const appWindow = getCurrentWindow();

const isMaximized = ref(false);
const openMenu = ref<"file" | "help" | null>(null);
let unlistenResized: UnlistenFn | null = null;

function toggleMenu(menu: "file" | "help"): void {
  openMenu.value = openMenu.value === menu ? null : menu;
}

function closeMenus(): void {
  openMenu.value = null;
}

function runItem(action: () => void): void {
  closeMenus();
  action();
}

function onDocumentClick(): void {
  closeMenus();
}

async function refreshMaximized(): Promise<void> {
  isMaximized.value = await appWindow.isMaximized();
}

onMounted(async () => {
  await refreshMaximized();
  unlistenResized = await appWindow.onResized(() => {
    void refreshMaximized();
  });
  document.addEventListener("click", onDocumentClick);
});

onBeforeUnmount(() => {
  unlistenResized?.();
  document.removeEventListener("click", onDocumentClick);
});
</script>

<template>
  <header class="titlebar" :class="{ maximized: isMaximized }" data-tauri-drag-region>
    <img src="/app-icon.png" alt="App Icon" class="window-icon" draggable="false" />

    <nav class="titlebar-menu-container" @click.stop>
      <div class="titlebar-menu">
        <button
          type="button"
          class="menu-button"
          :class="{ open: openMenu === 'file' }"
          @click.stop="toggleMenu('file')"
        >
          File
        </button>
        <button
          type="button"
          class="menu-button"
          :class="{ open: openMenu === 'help' }"
          @click.stop="toggleMenu('help')"
        >
          Help
        </button>
      </div>

      <div v-if="openMenu === 'file'" class="menu-dropdown">
        <button type="button" class="menu-item" @click.stop="runItem(() => emit('open-file'))">
          Open File...
        </button>
        <button type="button" class="menu-item" @click.stop="runItem(() => emit('open-files'))">
          Open Multiple Files...
        </button>
        <button type="button" class="menu-item" @click.stop="runItem(() => emit('open-directory'))">
          Open Folder...
        </button>
        <div class="menu-divider" />
        <button type="button" class="menu-item" @click.stop="runItem(() => appWindow.close())">
          Exit
        </button>
      </div>

      <div v-if="openMenu === 'help'" class="menu-dropdown help-menu">
        <button type="button" class="menu-item" @click.stop="runItem(() => ui.openAbout())">
          About
        </button>
      </div>
    </nav>

    <h1 class="titlebar-title" :title="titleText">{{ titleText }}</h1>

    <div class="titlebar-right">
      <div v-if="hasData" class="titlebar-actions">
        <button
          type="button"
          class="titlebar-action"
          title="Export Data"
          aria-label="Export Data"
          :disabled="isLoading"
          @click="emit('export')"
        >
          <i class="mdi mdi-export" />
        </button>
        <button
          type="button"
          class="titlebar-action"
          :class="{ active: isSqlMode }"
          :title="isSqlMode ? (sqlTime ? `Executed in ${sqlTime}ms - Exit SQL Mode` : 'Exit SQL Mode') : 'SQL Mode'"
          :aria-label="isSqlMode ? 'Exit SQL Mode' : 'SQL Mode'"
          @click="emit('sql')"
        >
          <i class="mdi mdi-database-search" />
        </button>
        <button
          type="button"
          class="titlebar-action"
          title="File Metadata"
          aria-label="File Metadata"
          @click="emit('metadata')"
        >
          <i class="mdi mdi-information-outline" />
        </button>
      </div>

      <div class="titlebar-controls">
        <button type="button" class="titlebar-button" aria-label="Minimize" @click="appWindow.minimize()">
          <i class="mdi mdi-window-minimize" />
        </button>
        <button
          type="button"
          class="titlebar-button"
          :aria-label="isMaximized ? 'Restore' : 'Maximize'"
          @click="appWindow.toggleMaximize()"
        >
          <i :class="isMaximized ? 'mdi mdi-window-restore' : 'mdi mdi-window-maximize'" />
        </button>
        <button
          type="button"
          class="titlebar-button close"
          aria-label="Close"
          @click="appWindow.close()"
        >
          <i class="mdi mdi-close" />
        </button>
      </div>
    </div>
  </header>
</template>

<style scoped>
.titlebar {
  height: 32px;
  background: rgb(var(--v-theme-surface));
  display: flex;
  align-items: center;
  border-bottom: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  flex-shrink: 0;
  position: relative;
  user-select: none;
  padding-left: 8px;
}

.titlebar-title {
  position: absolute;
  left: 50%;
  transform: translateX(-50%);
  max-width: 40%;
  text-align: center;
  margin: 0;
  font-size: 13px;
  font-weight: 500;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  pointer-events: none;
  font-family: inherit;
  z-index: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.window-icon {
  width: 18px;
  height: 18px;
  margin-right: 8px;
  opacity: 0.9;
  user-select: none;
  -webkit-user-select: none;
  flex-shrink: 0;
}

.titlebar-menu-container {
  position: relative;
  z-index: 20;
  height: 100%;
  display: flex;
  align-items: center;
}

.titlebar-menu {
  display: flex;
  align-items: center;
  height: 100%;
}

.menu-button {
  background: none;
  border: none;
  padding: 0 10px;
  margin: 0 2px;
  font: inherit;
  font-size: 13px;
  color: rgb(var(--v-theme-on-surface));
  display: inline-flex;
  align-items: center;
  height: 100%;
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.2s, color 0.2s;
}

.menu-button:hover,
.menu-button.open {
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  color: rgb(var(--v-theme-primary));
}

.menu-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  background: rgb(var(--v-theme-surface));
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 6px;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.4);
  min-width: 180px;
  padding: 4px;
  z-index: 30;
}

.menu-dropdown.theme-menu {
  left: 64px;
}

.menu-dropdown.help-menu {
  left: 48px;
}

.menu-item {
  display: flex;
  align-items: center;
  width: 100%;
  background: none;
  border: none;
  padding: 8px 12px;
  font: inherit;
  font-size: 13px;
  color: rgb(var(--v-theme-on-surface));
  text-align: left;
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.1s;
}

.menu-item:hover {
  background: rgb(var(--v-theme-primary));
  color: rgb(var(--v-theme-on-primary));
}

.menu-item.active {
  color: rgb(var(--v-theme-primary));
  font-weight: 600;
}

.menu-divider {
  height: 1px;
  background: rgba(var(--v-border-color), var(--v-border-opacity));
  margin: 4px 8px;
}

.titlebar-right {
  display: flex;
  align-items: center;
  height: 100%;
  margin-left: auto;
  z-index: 20;
}

.titlebar-actions {
  display: flex;
  align-items: center;
  height: 100%;
  gap: 2px;
  margin-right: 6px;
}

.titlebar-action {
  background: none;
  border: none;
  padding: 0;
  margin: 0;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 34px;
  height: 26px;
  cursor: pointer;
  transition: background 0.2s, color 0.2s;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  font-size: 0.9rem;
  border-radius: 4px;
}

.titlebar-action:hover:not(:disabled) {
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  color: rgb(var(--v-theme-on-surface));
}

.titlebar-action.active {
  background: rgba(var(--v-theme-primary), 0.2);
  color: rgb(var(--v-theme-primary));
}

.titlebar-action:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.titlebar-controls {
  display: flex;
  height: 100%;
}

.titlebar-button {
  background: none;
  border: none;
  padding: 0;
  margin: 0;
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 46px;
  height: 100%;
  cursor: pointer;
  transition: background 0.2s;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  font-size: 0.8rem;
}

.titlebar-button:hover {
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  color: rgb(var(--v-theme-on-surface));
}

.titlebar-button.close:hover {
  background: #e81123;
  color: #fff;
}
</style>
