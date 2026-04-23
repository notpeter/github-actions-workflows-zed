## GitHub Workflows for Zed

A [Zed](https://zed.dev) extension that adds a `GitHub Workflows` language
for GitHub Actions workflow files.

## What it provides

- A language named **GitHub Workflows** backed by Zed's built-in
  `tree-sitter-yaml` grammar (no extra YAML grammar is bundled or compiled).
- Syntax highlighting, bracket matching, outline, and text objects for YAML.
- Injection of [`tree-sitter-ghactions`](https://github.com/rmuir/tree-sitter-ghactions)
  inside any YAML string containing a `${{ ... }}` expression, giving rich
  highlighting of operators, identifiers, context accessors
  (`github`, `env`, `secrets`, ...), and builtin functions
  (`contains`, `fromJSON`, `hashFiles`, ...).
- Injection of `bash` into `run:` step values.

## Installation

Install the extension from Zed's extension registry (`zed: extensions`,
search for "GitHub Workflows"), or install it as a dev extension by cloning
this repo and running `zed: install dev extension`.

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

## License

MIT
