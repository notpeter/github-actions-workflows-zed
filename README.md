## GitHub YAML Zed Extension

[Zed](https://zed.dev) extension supporting GitHub YAML configuration files.

<img width="809" height="256" alt="GitHub Actions Workflow - Zed Screenshot with Syntax Highlighting " src="https://github.com/user-attachments/assets/fcb18b20-8b10-47ca-bac2-e6695e485877" />

## Features

- "GitHub YAML" language:
  - [tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml)
  - injection of [rmuir/tree-sitter-ghactions](https://github.com/rmuir/tree-sitter-ghactions) for `${{ }}` blocks
  - injection of [tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash) for `run:`
- JSON Schema registration with yaml-language-server supporting Auto-complete / Validation / Hover docs:
  - [GitHub Workflows](https://json.schemastore.org/github-workflow.json): `.github/workflows/*.{yml,yaml}`, `workflow-templates/*.yml`
  - [GitHub Actions](https://json.schemastore.org/github-action.json): `.github/actions/**/action.{yml,yaml}`
  - [GitHub Funding](https://json.schemastore.org/github-funding.json): `.github/FUNDING.yml`
  - [GitHub Discussions Templates](https://json.schemastore.org/github-discussion.json): `.github/DISCUSSION_TEMPLATE/*.yml`
  - [GitHub Issue Templates](https://json.schemastore.org/github-issue-forms.json): `.github/ISSUE_TEMPLATE/*.yml`
  - [GitHub Issue Configuration](https://json.schemastore.org/github-issue-config.json): `.github/ISSUE_TEMPLATE/config.yml`
  - [GitHub Release Configuration](https://json.schemastore.org/github-release-config.json): `.github/release.yml`
  - [GitHub Dependabot Configuration](https://www.schemastore.org/dependabot-2.0.json): `.github/dependabot.yml`

## Installation

Open `zed: extensions` from the command palette in Zed and search for "GitHub Workflows"

Or clone this repository and using `zed: install dev extension`.

## Required Settings

To associate GitHub Actions workflow files with this language, add the
following to your Zed `settings.json`:

```json
{
  "file_types": {
    "GitHub YAML": [
      "**/.github/workflows/*.{yml,yaml}",
      "**/.github/actions/**/action.{yml,yaml}",
      "**/.github/{ISSUE_TEMPLATE,DISCUSSION_TEMPLATE}/*.yml",
      "**/.github/{FUNDING,release}.yml",
      "**/.github/dependabot.{yml,yaml}",
      "**/workflow-templates/*.yml"
    ]
  }
}
```

### Alternatives

Note: It may be helpful to add `# yaml-language-server: $schema=` directives to the top of your files.
With this, editors configured with `yaml-language-server` (Zed, NeoVim, VSCode, etc) will also
get automatic schema association with schemas from [JSON SchemaStore](https://schemastore.org).
Make sure to substitue the correct Schema type for the file in question from the list above.

For a GitHub Workflow file, `**/.github/workflows/*.yml` use:

```yaml
# yaml-language-server: $schema=https://json.schemastore.org/github-workflow.json
```

### yaml-language-server settings

The extension registers its own `yaml-language-server` instance under the LSP name `github-yaml-language-server`.
Zed reserves the bare `yaml-language-server` name for its built-in YAML adapter).

User settings are deep-merged on top of the bundled defaults, so you can add your own
schemas or disable features without losing the bundled GitHub Actions schemas.

See [Zed YAML Language Docs](https://zed.dev/docs/languages/yaml) and
[yaml-language-server settings docs](https://github.com/redhat-developer/yaml-language-server?tab=readme-ov-file#language-server-settings)
for supported settings.

```json
{
  "lsp": {
    "github-yaml-language-server": {
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
