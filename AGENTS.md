## CLAUDE.md

## What this is

A [Zed](https://zed.dev) extension for GitHub YAML files.

Extension provides:
- `yaml-language-server` as `github-yaml-language-server` (`src/github_yaml.rs`)
- json schema associations for those language servers (`src/github_yaml.rs`)
- `GitHub Workflow` language for Zed
- `Code Owners` language for Zed

## Commands

- `./scripts/update.sh` Pulls `.scm` files from upstream repos and pre-processes/formats them.
- `cargo fmt`

## Architecture

### Languages (`languages/*/`)

Each language has a `config.toml` and tree-sitter `.scm` queries:

languages/github_workflow: "GitHub Workflow"
  - Same as Zed's built-in YAML and queries
  - Modified `injections.scm` for GitHub Workflow specific injections

languages/ghactions: "ghactions" (hidden=true)
  - Grammar for `${{ }}` expression bodies. Injected into YAML scalars.

languages/nim_format_string: "nim_format_string" (hidden=true)
  - Parses `{0}`-style placeholders inside ghactions `format()` calls.
  - Upstream is Nim-specific but sufficiently generic for our format-strings

languages/codeowners: "Code Owners"
  - `CODEOWNERS` files

### Injection chain

github_workflow/injections.scm:
  - YAML scalar containing `${{ ... }}` → `ghactions`
  - `run:` values → `bash`
  - `script:` values → `javascript` (on `actions/github-script` steps)

ghactions/injections:
  - `format('hi {0}', ...)` string literals → `nim_format_string`

### Rust extension (`src/github_yaml.rs`)

Rust edition 2024.
Builds to a `wasm32-wasip2` cdylib (`extension.wasm`).

Two jobs:

1. Install and launch `yaml-language-server` via Zed's npm helpers.
2. Serve `language_server_workspace_configuration` by embedding (`include_str!`) the JSON files
   in `schemas/`
   - `yaml-language-server` -> `schemas/yaml-language-server.json`
   - `json-language-server` -> `schemas/json-language-server.json`
   - `github-yaml-language-server` -> `schemas/github-yaml-language-server.json`

The LSP is registered under the Zed-internal name `github-yaml-language-server` (see `extension.toml`)
but `language_ids` maps `"GitHub YAML" = "yaml"` so `yaml-language-server` still recognizes the document.

### Notes

`extension.toml` pins each non-yaml grammar to a specific upstream commit.
Changing a pin is a deliberate decision — remember to re-run `scripts/update.sh`

## Gotchas

- `languages/github_workflow/injections.scm` and `languages/nim_format_string/injections.scm`
  Are intentionally NOT synced by `update.sh` as they're extension-specific.
- Zed doesn't honor tree-sitter's `(#set! priority N)`; `update.sh` strips it from ghactions highlights
