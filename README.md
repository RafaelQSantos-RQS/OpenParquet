# OpenParquet

A modern, high-performance desktop viewer for **Apache Parquet** files with SQL
queries, built with Tauri v2 (Rust + DuckDB) and Vue 3 + Vuetify.

## Features

- Open a single Parquet file, an entire folder, or multiple files at once
- Paginated table view with sortable columns and type hints
- **SQL mode**: query the opened dataset directly with DuckDB
- Export data to **CSV, JSON or Parquet** (full table or current query result)
- File metadata panel (schema, row count, file list)
- Recent files sidebar
- Dark-only theme, custom title bar with in-window actions
- Drag & drop files/folders to open

## Tech Stack

| Layer | Tech |
|---|---|
| Frontend | Vue 3, Vite, Vuetify 4, Pinia, TypeScript (strict) |
| Backend | Tauri v2, Rust, DuckDB |
| Package manager | [Bun](https://bun.sh) |

## Getting Started

```bash
bun install
bun run tauri dev   # Desktop app with hot reload (backend + frontend)
```

Frontend only (Vite dev server on port 1420):

```bash
bun run dev
```

## Build & Validation

```bash
bun run build               # Frontend: vue-tsc --noEmit + vite build
cargo clippy -- -D warnings # Backend: strict Rust lint (warnings = errors)
bun run tauri build         # Production binary
```

## Project Structure

```
src/            # Vue 3 frontend (components, composables, Pinia stores)
src-tauri/      # Rust/Tauri v2 backend (IPC commands, DuckDB logic, validation)
scripts/        # version sync hook (Cargo.toml ↔ package.json)
```

The backend exposes 4 dataset-centric commands via IPC
(`open_dataset`, `get_page`, `run_sql`, `export_dataset`) — see
[AGENTS.md](./AGENTS.md) for the full contract.

## Versioning

`npm version patch|minor|major` bumps both `package.json` and
`src-tauri/Cargo.toml` in a single action (via git hooks).
