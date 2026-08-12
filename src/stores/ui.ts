import { ref } from "vue";
import { defineStore } from "pinia";

export const useUiStore = defineStore("ui", () => {
  const showAbout = ref(false);
  const showSql = ref(false);
  const showExport = ref(false);
  const isExporting = ref(false);
  const isDragging = ref(false);

  function openAbout(): void {
    showAbout.value = true;
  }
  function closeAbout(): void {
    showAbout.value = false;
  }
  function openSql(): void {
    showSql.value = true;
  }
  function closeSql(): void {
    showSql.value = false;
  }
  function openExport(): void {
    showExport.value = true;
  }
  function closeExport(): void {
    showExport.value = false;
  }
  function setDragging(value: boolean): void {
    isDragging.value = value;
  }
  function setExporting(value: boolean): void {
    isExporting.value = value;
  }

  return {
    showAbout,
    showSql,
    showExport,
    isExporting,
    isDragging,
    openAbout,
    closeAbout,
    openSql,
    closeSql,
    openExport,
    closeExport,
    setDragging,
    setExporting,
  };
});
