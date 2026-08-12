# AGENTS.md — OpenParquet

Instructions for code agents working in this repository.

## Overview

OpenParquet is a desktop viewer for **Apache Parquet** files with SQL queries,
built with **Tauri v2** (Rust backend + DuckDB) and **Vue 3 + Vite + Vuetify 4 + Pinia**
(no meta-framework). The architecture is backend-first: 4 dataset-centric Tauri commands
expose all data logic via IPC.

## Commands

### Development
```bash
bun run tauri dev          # Desktop app with hot reload (backend + frontend)
bun run dev                # Frontend only (Vite, port 1420)
```

### Build & Verification
```bash
bun run build              # Frontend: vue-tsc --noEmit + vite build
cargo clippy -- -D warnings  # Backend: strict Rust lint (warnings = errors)
bun run tauri build        # Production binary
```

### Versioning
```bash
npm version patch|minor|major  # SINGLE ACTION — bumps package.json AND Cargo.toml (hooks)
```
`tauri.conf.json` uses `"version": "../package.json"` (single source of truth).
`preversion/version/postversion` hooks (scripts/sync-version.js) sync
`src-tauri/Cargo.toml`. The metainfo.xml lists versions manually.

## Project Structure

```
├── src/                     # Vue 3 frontend (script setup + Composition API, strict TS)
│   ├── main.ts             # Bootstrap: Pinia + Vuetify (dark-only theme) + @mdi/font
│   ├── App.vue             # Shell: workspace, sidebar (Recent Files), main area
│   ├── components/         # UI: Titlebar, AppLayout, DataTable, MetadataPanel,
│   │                       #     SqlModal, ExportModal, AboutModal, RecentFiles
│   ├── composables/        # useDataset (dataset state/actions), useDragDrop
│   ├── stores/             # Pinia: ui, recents (+ preferences via plugin-store)
│   └── types.ts            # TS contract of the backend (camelCase)
├── src-tauri/              # Rust/Tauri v2 backend
│   ├── src/
│   │   ├── lib.rs          # Builder: plugins (store/opener/dialog) + invoke_handler
│   │   ├── commands.rs     # 4 IPC commands (open_dataset, get_page, run_sql, export_dataset)
│   │   ├── db_logic.rs     # DuckDB: schema, pages, queries, export (value formatting)
│   │   ├── models.rs       # Serializable structs (serde camelCase)
│   │   ├── validation.rs   # Validation at the trust boundary (paths, forbidden SQL)
│   │   ├── error.rs        # AppError (thiserror + Serialize) — becomes a string on the frontend
│   │   └── state.rs        # AppState: Mutex<duckdb::Connection>
│   └── tauri.conf.json     # decorations:false, version: ../package.json
├── scripts/sync-version.js # Hook syncing Cargo.toml ↔ package.json
└── .tmp/old_code/          # Backup of the old project (Svelte) — reference only
```

## API Architecture (contract — DO NOT break)

4 dataset-centric commands, receiving and returning **camelCase** (serde rename_all):

| Command | Args | Returns |
|---|---|---|
| `open_dataset` | `source` | `DatasetInfo { schema, totalRows, files }` |
| `get_page` | `source, page, pageSize, sortCol?, sortOrder?` | `PageData` (rows) |
| `run_sql` | `source, query, page, pageSize` | `QueryResult { schema, rows, executionTimeMs, totalRows }` |
| `export_dataset` | `source, query, outputPath, format` | `()` |

`source` is a `SourceDescriptor` tagged union:
`{ type: 'file', path } | { type: 'dir', path } | { type: 'list', paths }`.

Rules:
- Commands return `AppResult<T>` (AppError implements Serialize — becomes an error string on the frontend, shown via snackbar).
- Path and query validation on the backend (validation.rs): absolute paths, `.parquet`, no `DROP/DELETE/TRUNCATE/ALTER/GRANT/REVOKE`.
- Do not create new commands keyed by path (file_path, directory_path...) — use `SourceDescriptor`.
- The frontend consumes data via `useDataset()` (single composable) — do not create parallel data stores.

## Code Style

- **Comments in English** (consistent with the current code); no obvious comments.
- **Conventional Commits**: `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`.
- **Frontend**: `<script setup lang="ts">`, Composition API, presentational components
  receive state via props and emit events; data logic lives in `useDataset`.
- **Backend**: small pure functions in `db_logic`, validation in `validation`,
  no `.map_err` boilerplate (use `?` with AppError `#[from]`).
- **Vuetify 4 patterns**: components auto-imported via `vite-plugin-vuetify`
  (no manual imports), `<v-app>` as root in App.vue, **dark-only theme** (no theme store),
  CSS vars `rgb(var(--v-theme-*))` / `rgba(var(--v-border-color), var(--v-border-opacity))`.

## Testing

No automated test suite. Validation = `bun run build` (strict vue-tsc)
+ `cargo clippy -- -D warnings` + manual testing with `bun run tauri dev`
(E2E criteria in `.tmp/context/refactor-vue-tauri.md`).
