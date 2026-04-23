## GitHub Workflows for Zed

A [Zed](https://zed.dev) extension that adds a `GitHub Workflows` language
for GitHub Actions workflow files.

## What it provides

- A language named **GitHub Workflows** backed by Zed's built-in
  `tree-sitter-yaml` grammar (no extra YAML grammar is bundled or compiled).
- Syntax highlighting, bracket matching, outline, and text objects for YAML.
- Injection of [`rmuir/tree-sitter-ghactions`](https://github.com/rmuir/tree-sitter-ghactions)
  inside any YAML string containing a `${{ ... }}` expression, giving rich
  highlighting of operators, identifiers, context accessors
  (`github`, `env`, `secrets`, ...), and builtin functions
  (`contains`, `fromJSON`, `hashFiles`, ...).
- Injection of `bash` into `run:` step values.
- Automatic registration of Red Hat's
  [`yaml-language-server`](https://github.com/redhat-developer/yaml-language-server)
  for `GitHub Workflows` buffers, pre-configured with the
  [SchemaStore](https://www.schemastore.org/) JSON Schemas for
  GitHub Actions workflows (`github-workflow.json`) and composite/custom
  actions (`github-action.json`). This gives you completion, hover docs,
  and validation without any manual LSP setup.

## Installation

Install the extension from Zed's extension registry (`zed: extensions`,
search for "GitHub Workflows"), or install it as a dev extension by cloning
this repo and running `zed: install dev extension`.

The first time you open a workflow file, Zed will download
`yaml-language-server` via `npm` into the extension's working directory.
A working `node` binary (managed by Zed) is required.

## Usage

To associate GitHub Actions workflow files with this language, add the
following to your Zed `settings.json`:

```json
{
  "file_types": {
    "GitHub Workflows": [
      "**/.github/workflows/*.yml",
      "**/.github/workflows/*.yaml",
      "**/.github/actions/**/action.yml",
      "**/.github/actions/**/action.yaml"
    ]
  }
}
```

### Customizing the YAML language server

The extension ships with sensible defaults for `yaml-language-server`.
User settings under `lsp."yaml-language-server".settings.yaml` are deep-merged
on top of those defaults, so you can add your own schemas or disable features
without losing the bundled GitHub Actions schemas. For example:

```json
{
  "lsp": {
    "yaml-language-server": {
      "settings": {
        "yaml": {
          "schemas": {
            "https://json.schemastore.org/dependabot-2.0.json": [
              ".github/dependabot.yml",
              ".github/dependabot.yaml"
            ]
          },
          "format": { "enable": true }
        }
      }
    }
  }
}
```

## License

MIT
