import { LazyStore } from "@tauri-apps/plugin-store";

let store: LazyStore | null = null;

export function getStore(): LazyStore {
  if (!store) {
    store = new LazyStore("preferences.json");
  }
  return store;
}
