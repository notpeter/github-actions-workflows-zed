use std::{env, fs};
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result};

const PACKAGE_NAME: &str = "yaml-language-server";
const SERVER_PATH: &str = "node_modules/yaml-language-server/bin/yaml-language-server";

struct GithubActionsExtension {
    did_find_server: bool,
}

impl GithubActionsExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, server_id: &zed::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

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
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id)?;
        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
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

        if let Ok(lsp_settings) = LspSettings::for_worktree("github-yaml-language-server", worktree)
        {
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
