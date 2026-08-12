import { ref } from "vue";
import { defineStore } from "pinia";
import { getStore } from "./preferences";

export type RecentEntry = {
  type: "file" | "dir" | "list";
  paths: string[];
};

const KEY = "recent_entries";
const MAX_ENTRIES = 10;

export const useRecentsStore = defineStore("recents", () => {
  const entries = ref<RecentEntry[]>([]);
  const loading = ref(false);

  function label(entry: RecentEntry): string {
    if (entry.type === "list") return `${entry.paths.length} files`;
    return entry.paths[0] ?? "";
  }

  function isSame(a: RecentEntry, b: RecentEntry): boolean {
    return (
      a.type === b.type &&
      a.paths.length === b.paths.length &&
      a.paths.every((p, i) => p === b.paths[i])
    );
  }

  async function load(): Promise<void> {
    loading.value = true;
    try {
      const val = await getStore().get<RecentEntry[]>(KEY);
      // ponytail: legacy (string[] under 'recent_files') uses another key and is ignored
      entries.value = Array.isArray(val) ? val : [];
    } catch (e) {
      console.error("Error reading recents:", e);
      entries.value = [];
    } finally {
      loading.value = false;
    }
  }

  async function persist(): Promise<void> {
    try {
      await getStore().set(KEY, entries.value);
      await getStore().save();
    } catch (e) {
      console.error("Error saving recents:", e);
    }
  }

  async function add(entry: RecentEntry): Promise<void> {
    entries.value = [
      entry,
      ...entries.value.filter((e) => !isSame(e, entry)),
    ].slice(0, MAX_ENTRIES);
    await persist();
  }

  async function remove(index: number): Promise<void> {
    entries.value = entries.value.filter((_, i) => i !== index);
    await persist();
  }

  async function clear(): Promise<void> {
    entries.value = [];
    await persist();
  }

  return { entries, loading, load, add, remove, clear, label };
});
