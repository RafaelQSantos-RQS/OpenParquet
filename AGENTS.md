# AGENTS.md - OpenParquet

Coding agent instructions for working in this repository.

## Project Overview

OpenParquet is a modern Parquet file viewer built with **Tauri v2** (Rust backend) + **Svelte 5** + **TypeScript** frontend. The backend uses **DuckDB** for high-performance Parquet handling and SQL queries.

---

## Build, Lint & Test Commands

### Development
```bash
npm run tauri dev          # Start dev server with hot reload
npm run dev                # Frontend only (Vite dev)
```

### Build
```bash
npm run build              # Build frontend only (Vite)
npm run tauri build        # Build production binary (all platforms)
make build                 # Same as npm run tauri build
```

### Lint & Type Check
```bash
npm run check              # Svelte/TypeScript type checking
npm run check:watch        # Type checking in watch mode
cargo clippy -- -D warnings  # Rust linting (strict: warnings as errors)
make check                 # Runs both npm run check and cargo clippy
```

### Testing
**No tests currently implemented.** E2E tests are planned (see roadmap in README.md).
When tests are added, document commands here.

---

## Project Structure

```
├── src/                    # Svelte frontend
│   ├── lib/               # Shared utilities & constants
│   │   ├── constants.ts   # App metadata, social links
│   │   └── preferences.ts # Theme & recent files (Tauri Store)
│   ├── routes/
│   │   ├── +page.svelte   # Main application page
│   │   ├── +layout.ts     # SSR disabled (export const ssr = false)
│   │   ├── page.css       # Main styles
│   │   └── components/    # UI components (one folder per component)
│   └── app.html           # HTML template
├── src-tauri/             # Rust backend
│   └── src/
│       ├── lib.rs         # Tauri app setup, plugin registration
│       ├── main.rs        # Entry point
│       ├── commands.rs    # Tauri IPC commands (invoked from frontend)
│       ├── db_logic.rs    # DuckDB operations
│       └── models.rs      # Data structures (ColumnInfo, PageData, etc.)
├── static/                # Static assets
├── package.json           # npm scripts & frontend dependencies
├── Makefile               # Convenient command shortcuts
└── tsconfig.json          # TypeScript config (strict mode enabled)
```

---

## Code Style Guidelines

### General

- **Language**: Comments in Portuguese (matches existing codebase).
- **No unnecessary comments** in code - write self-documenting code.
- **Conventional Commits**: Use prefixes like `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`.

---

### Frontend (TypeScript / Svelte)

#### Imports
```typescript
// 1. External packages first (alphabetically)
import { invoke } from "@tauri-apps/api/core";
import { onMount } from "svelte";

// 2. Internal aliases ($lib)
import { APP_INFO } from "$lib/constants";

// 3. Relative imports (components)
import Titlebar from "./components/Titlebar/Titlebar.svelte";
```

#### Naming Conventions
- **Variables/Functions**: `camelCase` (e.g., `loadParquetFile`, `sortOrder`)
- **Components/Types/Interfaces**: `PascalCase` (e.g., `ColumnInfo`, `FileMetadata`)
- **Constants**: `SCREAMING_SNAKE_CASE` for true constants, `camelCase` for config objects
- **Files**: `PascalCase.svelte` for components, `camelCase.ts` for utilities

#### TypeScript
- **Strict mode enabled** - no `any` unless absolutely necessary.
- Use **type annotations** for function parameters and return types.
- Prefer `interface` for object shapes, `type` for unions/primitives.
- Use `as` casts sparingly; prefer type guards.

```typescript
// Good
interface FileMetadata {
  file_path: string;
  total_rows: number;
  schema: ColumnInfo[];
}

// Export types from module context for reuse
<script context="module" lang="ts">
  export type ColumnInfo = { name: string; type: string };
</script>
```

#### Svelte Components
- One component per folder with matching CSS file: `ComponentName/ComponentName.svelte`
- Use `<script lang="ts">` for TypeScript.
- Export props with defaults:
  ```svelte
  <script lang="ts">
    export let schema: ColumnInfo[] = [];
    export let onsort: (col: string) => void = () => {};
  </script>
  ```
- Component styles: separate CSS file via `<style src="./ComponentName.css">`

#### Error Handling
```typescript
try {
  const result = await invoke<QueryResult>("run_sql", { ... });
  // handle success
} catch (e) {
  console.error("Erro na query:", e);
  errorMsg = `Erro na query: ${e}`;
}
```

---

### Backend (Rust)

#### Naming Conventions
- **Functions/Variables**: `snake_case` (e.g., `get_schema_from_db`, `file_path`)
- **Structs/Enums/Traits**: `PascalCase` (e.g., `ColumnInfo`, `QueryResult`)
- **Constants**: `SCREAMING_SNAKE_CASE`

#### Tauri Commands
- Use `#[tauri::command(rename_all = "camelCase")]` for JS-compatible naming.
- Return `Result<T, String>` for error handling.
- Use async for commands that do I/O:

```rust
#[tauri::command(rename_all = "camelCase")]
pub async fn get_file_metadata(file_path: String) -> Result<FileMetadata, String> {
    // ...
}
```

#### Error Handling Pattern
```rust
fn db_err(e: duckdb::Error) -> String {
    e.to_string()
}

// Usage: convert library errors to String
let conn = Connection::open_in_memory().map_err(db_err)?;
```

#### Structs & Serialization
```rust
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,  // Avoid reserved keyword
}
```

#### Modules
- Keep related functionality together.
- `commands.rs` - Tauri IPC handlers.
- `db_logic.rs` - Database operations (pure functions, no Tauri dependency).
- `models.rs` - Data structures only.

---

## Git Workflow

### Commit Messages
Follow [Conventional Commits](https://www.conventionalcommits.org/):
```
feat: adicionar filtro de colunas
fix: corrigir erro de drag and drop
refactor: extrair lógica de exportação para db_logic
docs: atualizar roadmap
chore: atualizar dependências
```

### Branches
- Use descriptive names: `feat/sql-mode`, `fix/pagination-bug`, `refactor/export-logic`

---

## Key Dependencies

### Frontend
- **@tauri-apps/api** - Core Tauri APIs (`invoke`, event system)
- **@tauri-apps/plugin-dialog** - File open/save dialogs
- **@tauri-apps/plugin-store** - Persistent preferences
- **svelte** / **@sveltejs/kit** - UI framework

### Backend
- **tauri** - Desktop app framework
- **duckdb** - Embedded SQL database with Parquet support
- **serde / serde_json** - Serialization
- **chrono** - Date/time handling

---

## Common Tasks

### Adding a new Tauri command
1. Define the function in `src-tauri/src/commands.rs` with `#[tauri::command]`
2. Register it in `src-tauri/src/lib.rs` inside `invoke_handler![]`
3. Call from frontend: `await invoke("command_name", { arg1: value })`

### Adding a new UI component
1. Create folder: `src/routes/components/ComponentName/`
2. Add `ComponentName.svelte` and `ComponentName.css`
3. Import and use in parent component

### Adding a preference
1. Add getter/setter in `src/lib/preferences.ts`
2. Use `LazyStore` from `@tauri-apps/plugin-store`
