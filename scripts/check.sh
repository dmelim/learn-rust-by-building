#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
bash scripts/check-links.sh

if command -v node >/dev/null 2>&1; then
  node scripts/check-site.mjs
else
  printf 'Node.js not found; skipped course-reader checks.\n'
fi
