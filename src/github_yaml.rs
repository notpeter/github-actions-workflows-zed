use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result};

struct GithubActionsExtension {}

impl GithubActionsExtension {}

fn merge(a: &mut serde_json::Value, b: serde_json::Value) {
    match (a, b) {
        (serde_json::Value::Object(a), serde_json::Value::Object(b)) => {
            for (key, value) in b {
                merge(a.entry(key).or_insert(serde_json::Value::Null), value);
            }
        }
        (a, b) => *a = b,
    }
}

impl zed::Extension for GithubActionsExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let lsp_name = language_server_id.as_ref();
        if lsp_name != "yaml-language-server" {
            return Ok(None);
        }
        let mut yaml_settings = serde_json::json!({
            "schemas": {
                "https://json.schemastore.org/github-workflow.json": [
                    ".github/workflows/*.yml",
                    ".github/workflows/*.yaml",
                    "workflow-templates/*.yml"
                ],
                "https://json.schemastore.org/github-action.json": [
                    ".github/actions/**/action.yml",
                    ".github/actions/**/action.yaml"
                ],
                "https://json.schemastore.org/github-funding.json": [
                    ".github/FUNDING.yml"
                ],
                "https://json.schemastore.org/github-discussion.json": [
                    ".github/DISCUSSION_TEMPLATE/*.yml"
                ],
                "https://json.schemastore.org/github-issue-forms.json": [
                    "!.github/ISSUE_TEMPLATE/config.yml",
                    ".github/ISSUE_TEMPLATE/*.yml"
                ],
                "https://json.schemastore.org/github-issue-config.json": [
                    ".github/ISSUE_TEMPLATE/config.yml"
                ],
                "https://json.schemastore.org/github-release-config.json": [
                    ".github/release.yml"
                ],
                "https://json.schemastore.org/dependabot-2.0.json": [
                    ".github/dependabot.yml",
                    ".github/dependabot.yaml"
                ],
                "https://raw.githubusercontent.com/citation-file-format/citation-file-format/1.2.0/schema.json": [
                    "CITATION.cff"
                ]
            },
            "validate": true,
            "hover": true,
            "completion": true,
            "format": { "enable": false }
        });

        if let Ok(lsp_settings) = LspSettings::for_worktree(lsp_name, worktree) {
            if let Some(user_settings) = lsp_settings.settings {
                if let Some(user_yaml) = user_settings.get("yaml").cloned() {
                    merge(&mut yaml_settings, user_yaml);
                } else {
                    merge(&mut yaml_settings, user_settings);
                }
            }
        }

        Ok(Some(serde_json::json!({ "yaml": yaml_settings })))
    }
}

zed::register_extension!(GithubActionsExtension);
