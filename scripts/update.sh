#!/usr/bin/env bash
# Sync generic YAML tree-sitter queries from zed-industries/zed main tree into
# languages/github_yaml/. Skips injections.scm and config.toml, which are
# customized for this extension.
set -euo pipefail

REF="${1:-main}"
BASE="https://raw.githubusercontent.com/zed-industries/zed/${REF}/crates/grammars/src/yaml"
DEST="$(cd "$(dirname "$0")/.." && pwd)/languages/github_yaml"

FILES=(
  brackets.scm
  highlights.scm
  outline.scm
  overrides.scm
  redactions.scm
  textobjects.scm
)

echo "Syncing from ${BASE}"
for f in "${FILES[@]}"; do
  echo "  -> ${f}"
  curl -fsSL "${BASE}/${f}" -o "${DEST}/${f}"
done

echo "Formatting files..."
ts_query_ls format languages/*/*.scm

echo
echo "Skipped (extension-specific, not synced):"
echo "  - injections.scm  (ghactions expression + bash run: injections)"
echo "    ${BASE}/injections.scm"
echo "  - config.toml     (language name, indent rules)"
echo "    ${BASE}/config.toml"
