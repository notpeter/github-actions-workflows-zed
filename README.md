## GitHub YAML Zed Extension

[Zed](https://zed.dev) extension supporting GitHub YAML configuration files.

<img width="809" height="256" alt="GitHub Actions Workflow - Zed Screenshot with Syntax Highlighting " src="https://github.com/user-attachments/assets/fcb18b20-8b10-47ca-bac2-e6695e485877" />

## Features

- Adds `GitHub Workflow` language:
  - base [tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml) identical to Zed YAML language
  - injection of [tree-sitter-ghactions](https://github.com/rmuir/tree-sitter-ghactions) for `${{ }}` blocks
  - injection of [tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash) for `run:` blocks
  - injection of [tree-sitter-javascript](https://github.com/tree-sitter/tree-sitter-javascript) for [actions/github-script](https://github.com/actions/github-script) `script:` blocks
  - injection of [tree-sitter-nim-format-string](https://github.com/aMOPel/tree-sitter-nim-format-string) for format placeholders: `${{ format('hi {0}', 'Bob') }}`
- Adds `CODEOWNERS` language
  - uses [tree-sitter-codeowners](https://github.com/lukasmalkmus/tree-sitter-codeowners)
- JSON Schemas with `yaml-language-server` to support auto-complete, validation and hover docs for:
  - [GitHub Workflow](https://json.schemastore.org/github-workflow.json): `.github/workflows/*.{yml,yaml}`, `workflow-templates/*.yml`
  - [GitHub Action](https://json.schemastore.org/github-action.json): `.github/actions/**/action.{yml,yaml}`
  - [GitHub Funding](https://json.schemastore.org/github-funding.json): `.github/FUNDING.yml`
  - [GitHub Discussions Templates](https://json.schemastore.org/github-discussion.json): `.github/DISCUSSION_TEMPLATE/*.yml`
  - [GitHub Issue Templates](https://json.schemastore.org/github-issue-forms.json): `.github/ISSUE_TEMPLATE/*.yml`
  - [GitHub Issue Configuration](https://json.schemastore.org/github-issue-config.json): `.github/ISSUE_TEMPLATE/config.yml`
  - [GitHub Release Configuration](https://json.schemastore.org/github-release-config.json): `.github/release.yml`
  - [GitHub Dependabot Configuration](https://www.schemastore.org/dependabot-2.0.json): `.github/dependabot.yml`
  - [Citation File Format](https://github.com/citation-file-format/citation-file-format): `CITATION.cff`

## Installation

Open `zed: extensions` from the command palette in Zed and search for `GitHub YAML`,

Or click  this link [zed://extension/github-yaml](zed://extension/github-yaml)

Or clone this repository and using `zed: install dev extension`.

## Required Settings

To take advantage of the improvements of the `GitHub Workflow` over plain
YAML we need to add some configuration to your Zed settings.json.

To associate `GitHub Workflow` language with your files add the following to your
Zed `settings.json` by launching  `zed: open settings file` from the command palette.

```jsonc
{
  "file_types": {
    "GitHub Workflow": [
      "**/.github/workflows/*.{yml,yaml}",
      "**/.github/actions/**/action.{yml,yaml}",
      "**/workflow-templates/*.yml",
    ],
  },
}
```

## Recommended Companion Extension

I also recommend the [Git Firefly extension](https://github.com/zed-extensions/git_firefly)
which provides tree-sitters for the following:

- Git Attributes: .gitattributes, .git/info/attributes, etc
- Git Config: .gitconfig, .gitmodules, .lfsconfig, config.worktree
- Git Ignore: .gitignore, .dockerignore, .npmignore, .prettierignore, etc
- Git Rebase: git-rebase-todo

Click here: [zed://extension/git-firefly](zed://extension/git-firefly) to install and then add the following settings:

```json
{
  "file_types": {
    // Git Firefly extension languages
    "Git Attributes": ["**/{git,.git,.git/info}/attributes"],
    "Git Config": ["*.gitconfig", "**/{git,.git/modules,.git/modules/*}/config"],
    "Git Ignore": ["**/{git,.git}/ignore", "**/.git/info/exclude"],
  }
}
```

### Alternatives

Note: It may be helpful to add `# yaml-language-server: $schema=` directives to the top of your files.
With this, editors configured with `yaml-language-server` (Zed, NeoVim, VSCode, etc) will also
get automatic schema association with schemas from [JSON SchemaStore](https://schemastore.org).
Make sure to substitue the correct Schema type for the file in question from the list above.

E.g. For a GitHub Actions Workflow file, `**/.github/workflows/*.yml` use:

```yaml
# yaml-language-server: $schema=https://json.schemastore.org/github-workflow.json
```

### yaml-language-server settings

The extension registers its own `yaml-language-server` instance under the LSP name `github-yaml-language-server`.

User settings are deep-merged on top of the bundled defaults, so you can add your own
schemas or disable features without losing the bundled GitHub Actions schemas.

For example:
```json
{
  "lsp": {
    "github-yaml-language-server": {
      "settings": {
        "yaml": {
          "schemas": {
            "https://json.schemastore.org/whatever.json": [
              "whatever/*.yml",
            ]
          },
          "format": { "enable": true }
        }
      }
    }
  }
}
```

See [Zed YAML Language Docs](https://zed.dev/docs/languages/yaml) and
[yaml-language-server settings docs](https://github.com/redhat-developer/yaml-language-server?tab=readme-ov-file#language-server-settings)
for supported settings.

## License

MIT
