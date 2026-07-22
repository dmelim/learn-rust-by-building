#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

port="${1:-8000}"
printf 'Course reader: http://localhost:%s\n' "$port"
printf 'Press Ctrl+C to stop.\n'

if command -v node >/dev/null 2>&1; then
  node site/static-server.mjs "$port"
elif command -v python3 >/dev/null 2>&1; then
  python3 -m http.server "$port"
elif command -v python >/dev/null 2>&1; then
  python -m http.server "$port"
elif command -v py >/dev/null 2>&1; then
  py -m http.server "$port"
else
  printf 'Python and Node.js were not found. Use the local-server feature in your code editor.\n' >&2
  exit 1
fi
