// Sincroniza a versão do Cargo.toml (src-tauri) com o package.json.
// Uso: node scripts/sync-version.js  →  hook npm (preversion/postversion)
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const pkg = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
const cargoPath = join(root, "src-tauri", "Cargo.toml");
const cargo = readFileSync(cargoPath, "utf8");

const updated = cargo.replace(
  /^version = ".*"$/m,
  `version = "${pkg.version}"`,
);

if (updated !== cargo) {
  writeFileSync(cargoPath, updated);
  console.log(`Cargo.toml version -> ${pkg.version}`);
}
