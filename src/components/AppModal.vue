<script setup lang="ts">
// Generic modal wrapper replicating the old_code Modal:
// backdrop (click closes) + window with header (centered title + X) + Esc closes.
import { onBeforeUnmount, onMounted } from "vue";

defineProps<{ title: string }>();

const emit = defineEmits<{ close: [] }>();

function close(): void {
  emit("close");
}

function onKeydown(e: KeyboardEvent): void {
  if (e.key === "Escape") close();
}

onMounted(() => {
  document.addEventListener("keydown", onKeydown);
});
onBeforeUnmount(() => {
  document.removeEventListener("keydown", onKeydown);
});
</script>

<template>
  <div class="modal-backdrop" role="button" tabindex="0" @click="close">
    <div
      class="modal-window"
      role="dialog"
      aria-modal="true"
      aria-labelledby="modal-title"
      tabindex="-1"
      @click.stop
    >
      <div class="modal-header">
        <h3 id="modal-title">{{ title }}</h3>
        <button class="close-btn" aria-label="Close" @click="close">
          <i class="mdi mdi-close" />
        </button>
      </div>
      <div class="modal-content">
        <slot />
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  padding: 1rem;
  box-sizing: border-box;
}

.modal-window {
  background: rgb(var(--v-theme-surface));
  border: 1px solid rgba(var(--v-border-color), var(--v-border-opacity));
  border-radius: 8px;
  width: 100%;
  max-width: 600px;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.3);
  display: flex;
  flex-direction: column;
  max-height: 90vh;
  overflow: hidden;
}

.modal-header {
  padding: 0.6rem 1.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  flex-shrink: 0;
}

.modal-header h3 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  line-height: 1.2;
  color: rgb(var(--v-theme-on-surface));
  text-align: center;
}

.close-btn {
  background: none;
  border: none;
  color: rgba(var(--v-theme-on-surface), var(--v-medium-emphasis-opacity));
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  display: flex;
  transition: all 0.2s;
  position: absolute;
  right: 1rem;
  top: 50%;
  transform: translateY(-50%);
}

.close-btn:hover {
  background: rgba(var(--v-theme-on-surface), var(--v-hover-opacity));
  color: rgb(var(--v-theme-on-surface));
}

.modal-content {
  padding: 1.5rem;
  padding-bottom: 2rem;
  color: rgb(var(--v-theme-on-surface));
  line-height: 1.6;
  overflow-y: auto;
  flex: 1;
}
</style>
