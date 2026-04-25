## GitHub YAML Zed Extension

[Zed](https://zed.dev) extension supporting GitHub YAML configuration files.

<img width="809" height="256" alt="GitHub Actions Workflow - Zed Screenshot with Syntax Highlighting " src="https://github.com/user-attachments/assets/fcb18b20-8b10-47ca-bac2-e6695e485877" />

Features of this extension

- Advanced `GitHub Workflow` YAML syntax highlighting (`${{ }}` and `run:` blocks).
- Schema definitions for 9 GitHub YAML configuration file formats
  - GitHub Workflow, Action, Funding, Dependabot, Release, Issues, etc
- Adds schema definitions for 12+ common YAML/JSON configuration file formats
  - `prettierrc`, `gitlab-ci.yml`, `renovate.json`, `vercel.json`, etc.

## Installation

Open `zed: extensions` from the command palette in Zed and search for `GitHub YAML`,

Or clone this repository and spawn `zed: install dev extension`.

## Required Settings

You must associate `GitHub Workflow` language with the appropriate globs:

1. Open Zed command palette (`cmd-shift-p` / `ctrl-shift-p`)
1. Spawn `zed::OpenSettingsFile`
1. Add the following to your Zed `settings.json`

```jsonc
{
  "file_types": {
    "GitHub Workflow": [
      "**/.github/workflows/*.{yml,yaml}",
      "**/.github/actions/**/action.{yml,yaml}",
    ],
  },
}
```

## Git Firefly Extension

I also recommend the excellent [Git Firefly extension](https://github.com/zed-extensions/git_firefly)
which provides Zed languages for `.gitattributes`, `.gitconfig,`, `.gitignore`, etc.

Don't forget to add it's `file_types` associations as well:

```json
{
  "file_types": {
    // Git Firefly extension
    "Git Attributes": ["**/{git,.git,.git/info}/attributes"],
    "Git Config": ["*.gitconfig", "**/{git,.git/modules,.git/modules/*}/config"],
    "Git Ignore": ["**/{git,.git}/ignore", "**/.git/info/exclude"]
  }
}
```

### Alternatives

> [!NOTE]
> Consider adding explicit `# $schema=` directives to the top of your YAML files and
> `"$schema": ""` in your JSON files so your editor can automatically validate schemas
> without requiring configuration like that provided by this extension.

For a GitHub Actions Workflow file use:

```yaml
# $schema=https://json.schemastore.org/github-workflow.json
```

Or in a `vercel.json`:

```json
{
  "$schema": "https://openapi.vercel.sh/vercel.json"
}
```

## Features

- Adds `GitHub Workflow` language:
  - base [tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml) identical to Zed YAML language
  - injection of [tree-sitter-ghactions](https://github.com/rmuir/tree-sitter-ghactions) for `${{ }}` blocks
  - injection of [tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash) for `run:` blocks
  - injection of [tree-sitter-javascript](https://github.com/tree-sitter/tree-sitter-javascript) for [actions/github-script](https://github.com/actions/github-script) `script:` blocks
  - injection of [tree-sitter-nim-format-string](https://github.com/aMOPel/tree-sitter-nim-format-string) for format placeholders: `${{ format('hi {0}', 'Bob') }}`
- JSON Schema settings to support auto-complete, validation and hover docs for:
  - [GitHub Workflow](https://json.schemastore.org/github-workflow.json): `.github/workflows/*.{yml,yaml}`
  - [GitHub Action](https://json.schemastore.org/github-action.json): `.github/actions/**/action.{yml,yaml}`
  - [GitHub Funding](https://json.schemastore.org/github-funding.json): `.github/FUNDING.yml`
  - [GitHub Discussions Templates](https://json.schemastore.org/github-discussion.json): `.github/DISCUSSION_TEMPLATE/*.yml`
  - [GitHub Issue Templates](https://json.schemastore.org/github-issue-forms.json): `.github/ISSUE_TEMPLATE/*.yml`
  - [GitHub Issue Configuration](https://json.schemastore.org/github-issue-config.json): `.github/ISSUE_TEMPLATE/config.yml`
  - [GitHub Release Configuration](https://json.schemastore.org/github-release-config.json): `.github/release.yml`
  - [GitHub Dependabot Configuration](https://www.schemastore.org/dependabot-2.0.json): `.github/dependabot.yml`
  - [Citation File Format](https://github.com/citation-file-format/citation-file-format): `CITATION.cff`
  - [GitLab CI](https://gitlab.com/gitlab-org/gitlab-foss/-/raw/master/app/assets/javascripts/editor/schema/ci.json): `.gitlab-ci.{yml,yaml}`, `*.gitlab-ci.{yml,yaml}`
  - [pre-commit-config](https://json.schemastore.org/pre-commit-config.json): `.pre-commit-config.{yml,yaml}`
  - [pre-commit-hooks](https://json.schemastore.org/pre-commit-hooks.json): `.pre-commit-hooks.{yml,yaml}`
  - [CircleCI](https://raw.githubusercontent.com/CircleCI-Public/circleci-yaml-language-server/refs/heads/main/schema.json): `.circleci/config.{yml,yaml}`
  - [Render](https://render.com/schema/render.yaml.json): `render.{yml,yaml}`
  - [Azure Pipelines](https://raw.githubusercontent.com/microsoft/azure-pipelines-vscode/master/service-schema.json): `azure-pipelines.{yml,yaml}`
  - [Buildkite](https://raw.githubusercontent.com/buildkite/pipeline-schema/main/schema.json): `buildkite.{yml,yaml,json}`
  - [GoReleaser](https://goreleaser.com/static/schema.json): `.goreleaser.{yml,yaml}`
  - [Vercel](https://openapi.vercel.sh/vercel.json): `vercel.json`
  - [MkDocs](https://json.schemastore.org/mkdocs-1.6.json): `mkdocs.{yml,yaml}`
  - [Renovate](https://docs.renovatebot.com/renovate-schema.json): `renovate.json`
  - [Prettier](https://json.schemastore.org/prettierrc.json): `.prettierrc`, `.prettierrc.{json,yml,yaml}`
  - [Biome](https://biomejs.dev/schemas/latest/schema.json): `biome.{json,jsonc}`

### LSP settings

The extension registers its own `yaml-language-server` instance under the LSP name
`github-yaml-language-server`. This is only used for the `GitHub Workflows` language.

The schema associations are injected as language server settings and can be found here:

- [schemas/yaml-language-server.json](schemas/yaml-language-server.json)
- [schemas/json-language-server.json](schemas/json-language-server.json)
- [schemas/github-yaml-language-server.json](schemas/github-yaml-language-server.json)

User settings are deep-merged on top of the bundled defaults, so you can add your own
schemas or disable features without losing the bundled schemas.

See [Zed YAML Language Docs](https://zed.dev/docs/languages/yaml) and
[yaml-language-server settings docs](https://github.com/redhat-developer/yaml-language-server?tab=readme-ov-file#language-server-settings)
for more.

## License

MIT
