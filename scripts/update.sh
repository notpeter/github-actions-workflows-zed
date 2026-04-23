#!/usr/bin/env bash
# Sync tree-sitter queries for each bundled language from their upstream repos.
#
#   github_yaml       <- zed-industries/zed (generic YAML queries)
#   ghactions         <- rmuir/tree-sitter-ghactions
#   nim_format_string <- neovim-treesitter/nvim-treesitter-queries-nim_format_string
#   codeowners        <- lukasmalkmus/tree-sitter-codeowners
#
# Extension-specific files (injections.scm for github_yaml, config.toml
# everywhere) are not synced. See the "Skipped" list at the end.
set -euo pipefail

REF="${1:-main}"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# --- github_yaml -------------------------------------------------------------
YAML_BASE="https://raw.githubusercontent.com/zed-industries/zed/${REF}/crates/grammars/src/yaml"
YAML_DEST="${ROOT}/languages/github_yaml"
YAML_FILES=(
  brackets.scm
  highlights.scm
  outline.scm
  overrides.scm
  redactions.scm
  textobjects.scm
)
echo "Syncing github_yaml from ${YAML_BASE}"
for f in "${YAML_FILES[@]}"; do
  echo "  -> ${f}"
  curl -fsSL "${YAML_BASE}/${f}" -o "${YAML_DEST}/${f}"
done

# --- ghactions ---------------------------------------------------------------
# Upstream highlights tag most patterns with `(#set! priority 101)` to win
# ties against bash injections in nvim-treesitter. Zed doesn't honor that
# predicate, so drop it (keeping the surrounding pattern's close paren).
GHA_BASE="https://raw.githubusercontent.com/rmuir/tree-sitter-ghactions/main/queries"
GHA_DEST="${ROOT}/languages/ghactions"
GHA_FILES=(
  highlights.scm
  injections.scm
)
echo "Syncing ghactions from ${GHA_BASE}"
for f in "${GHA_FILES[@]}"; do
  echo "  -> ${f}"
  curl -fsSL "${GHA_BASE}/${f}" \
    | sed -e 's|(#set! priority 101))|)|g' \
          -e '/^; note: all highlights .* priority 101/d' \
    > "${GHA_DEST}/${f}"
done

# --- nim_format_string -------------------------------------------------------
# Remap Neovim-only capture names to Zed-supported equivalents, and drop the
# `@none` marker (also Neovim-specific). injections.scm is intentionally not
# synced: upstream injects `nim` into placeholder expressions, which isn't
# applicable to the numeric positional args used by GitHub Actions' format().
NIM_BASE="https://raw.githubusercontent.com/neovim-treesitter/nvim-treesitter-queries-nim_format_string/main/queries"
NIM_DEST="${ROOT}/languages/nim_format_string"
echo "Syncing nim_format_string from ${NIM_BASE}"
echo "  -> highlights.scm"
curl -fsSL "${NIM_BASE}/highlights.scm" \
  | sed -e 's|@keyword\.conditional\.ternary|@operator|g' \
        -e 's|@variable\.member|@number|g' \
        -e 's| @none||g' \
  > "${NIM_DEST}/highlights.scm"

# --- codeowners --------------------------------------------------------------
CODEOWNERS_BASE="https://raw.githubusercontent.com/lukasmalkmus/tree-sitter-codeowners/main/queries"
CODEOWNERS_DEST="${ROOT}/languages/codeowners"
echo "Syncing codeowners from ${CODEOWNERS_BASE}"
echo "  -> highlights.scm"
curl -fsSL "${CODEOWNERS_BASE}/highlights.scm" -o "${CODEOWNERS_DEST}/highlights.scm"

echo "Formatting files..."
ts_query_ls format languages/*/*.scm

echo
echo "Skipped (extension-specific, not synced):"
echo "  - languages/github_yaml/injections.scm  (ghactions + bash run: + github-script injections)"
echo "  - languages/github_yaml/config.toml     (language name, indent rules)"
echo "  - languages/ghactions/config.toml       (hidden-language config)"
echo "  - languages/nim_format_string/injections.scm  (upstream injects nim; N/A here)"
echo "  - languages/nim_format_string/config.toml     (hidden-language config)"
echo "  - languages/codeowners/config.toml      (language name, path_suffixes)"
