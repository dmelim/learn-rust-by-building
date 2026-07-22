#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

failed=0
while IFS= read -r markdown_file; do
  while IFS= read -r target; do
    case "$target" in
      http://*|https://*|mailto:*|\#*) continue ;;
    esac

    target="${target%%#*}"
    target="${target//%20/ }"
    if [[ -n "$target" && ! -e "$(dirname "$markdown_file")/$target" ]]; then
      printf '%s: broken local link: %s\n' "$markdown_file" "$target" >&2
      failed=1
    fi
  done < <(sed -nE 's/.*\[[^]]+\]\(([^)]+)\).*/\1/p' "$markdown_file")
done < <(find . -type f -name '*.md' -not -path './target/*' -not -path './.git/*')

exit "$failed"
