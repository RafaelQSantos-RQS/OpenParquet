import { getCurrentWindow } from "@tauri-apps/api/window";
import type { UnlistenFn } from "@tauri-apps/api/event";
import { getActivePinia } from "pinia";
import { onUnmounted, ref, type Ref } from "vue";
import { useUiStore } from "../stores/ui";

export interface UseDragDrop {
  isDragging: Ref<boolean>;
  onDrop: (callback: (paths: string[]) => void) => Promise<void>;
}

/**
 * Drag-and-drop of files/folders on the Tauri v2 window.
 * Syncs isDragging with the ui store when Pinia is active;
 * works standalone (local ref only) otherwise.
 */
export function useDragDrop(): UseDragDrop {
  const isDragging = ref(false);
  const pinia = getActivePinia();
  let unlisten: UnlistenFn | null = null;

  function setDragging(value: boolean): void {
    isDragging.value = value;
    if (pinia) {
      useUiStore(pinia).setDragging(value);
    }
  }

  async function onDrop(callback: (paths: string[]) => void): Promise<void> {
    unlisten = await getCurrentWindow().onDragDropEvent((event) => {
      const { payload } = event;
      switch (payload.type) {
        case "enter":
          setDragging(true);
          break;
        case "over":
          break;
        case "drop":
          setDragging(false);
          callback(payload.paths);
          break;
        case "leave":
          setDragging(false);
          break;
      }
    });
  }

  onUnmounted(() => {
    unlisten?.();
    unlisten = null;
  });

  return { isDragging, onDrop };
}
