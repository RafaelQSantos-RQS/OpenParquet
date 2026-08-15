// Syncs the Cargo.toml/Cargo.lock version (src-tauri) with package.json.
// Usage: node scripts/sync-version.js  →  npm hook (preversion/version/postversion)
import { execSync } from "node:child_process";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const pkg = JSON.parse(readFileSync(join(root, "package.json"), "utf8"));
const cargoPath = join(root, "src-tauri", "Cargo.toml");
const lockPath = join(root, "src-tauri", "Cargo.lock");

function bumpVersion(filePath, version) {
  const content = readFileSync(filePath, "utf8");
  const updated = content.replace(
    /^version = ".*"$/m,
    `version = "${version}"`,
  );
  if (updated !== content) {
    writeFileSync(filePath, updated);
    console.log(`${filePath} version -> ${version}`);
  }
}

// Cargo.toml: top-level version field.
bumpVersion(cargoPath, pkg.version);

// Cargo.lock: version of the root package (name = "openparquet").
const lock = readFileSync(lockPath, "utf8");
const lockUpdated = lock.replace(
  /(name = "openparquet"\n)version = ".*"/,
  `$1version = "${pkg.version}"`,
);
if (lockUpdated !== lock) {
  writeFileSync(lockPath, lockUpdated);
  console.log(`${lockPath} version -> ${pkg.version}`);
}

// Stage the synced files so `npm version` includes them in its commit.
execSync(`git add ${cargoPath} ${lockPath}`, { stdio: "inherit" });