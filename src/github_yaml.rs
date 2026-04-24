use std::{env, fs};
use zed_extension_api::{self as zed, Result, serde_json, settings::LspSettings};

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
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let lsp_name = language_server_id.as_ref();
        if lsp_name != "github-yaml-language-server" && lsp_name != "yaml-language-server" {
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
                ],
                "https://gitlab.com/gitlab-org/gitlab-foss/-/raw/master/app/assets/javascripts/editor/schema/ci.json": [
                    ".gitlab-ci.yml",
                    ".gitlab-ci.yaml",
                    "*.gitlab-ci.yml",
                    "*.gitlab-ci.yaml"
                ],
                "https://json.schemastore.org/pre-commit-config.json": [
                    ".pre-commit-config.yml",
                    ".pre-commit-config.yaml"
                ],
                "https://json.schemastore.org/pre-commit-hooks.json": [
                    ".pre-commit-hooks.yml",
                    ".pre-commit-hooks.yaml"
                ],
                "https://raw.githubusercontent.com/CircleCI-Public/circleci-yaml-language-server/refs/heads/main/schema.json": [
                    ".circleci/config.yml",
                    ".circleci/config.yaml"
                ],
                "https://render.com/schema/render.yaml.json": [
                    "render.yml",
                    "render.yaml"
                ],
                "https://raw.githubusercontent.com/microsoft/azure-pipelines-vscode/master/service-schema.json": [
                    "azure-pipelines.yml",
                    "azure-pipelines.yaml",
                    ".azure-pipelines.yml",
                    ".azure-pipelines.yaml",
                    ".azure-pipelines/**/*.yml",
                    ".azure-pipelines/**/*.yaml"
                ],
                "https://raw.githubusercontent.com/buildkite/pipeline-schema/main/schema.json": [
                    "buildkite.yml",
                    "buildkite.yaml",
                    "buildkite.*.yml",
                    "buildkite.*.yaml",
                    ".buildkite/pipeline.yml",
                    ".buildkite/pipeline.yaml",
                    ".buildkite/pipeline.*.yml",
                    ".buildkite/pipeline.*.yaml"
                ],
                "https://goreleaser.com/static/schema.json": [
                    ".goreleaser.yml",
                    ".goreleaser.yaml"
                ],
                "https://json.schemastore.org/mkdocs-1.6.json": [
                    "mkdocs.yml",
                    "mkdocs.yaml"
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

    fn language_server_additional_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        target_language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        if target_language_server_id.as_ref() != "json-language-server" {
            return Ok(None);
        }
        Ok(Some(serde_json::json!({
            "json": {
                "schemas": [
                    {
                        "fileMatch": [
                            "renovate.json",
                            ".github/renovate.json",
                            ".gitlab/renovate.json",
                            ".renovaterc",
                            ".renovaterc.json"
                        ],
                        "url": "https://docs.renovatebot.com/renovate-schema.json"
                    },
                    {
                        "fileMatch": [
                            "buildkite.json",
                            "buildkite.*.json",
                            ".buildkite/pipeline.json",
                            ".buildkite/pipeline.*.json"
                        ],
                        "url": "https://raw.githubusercontent.com/buildkite/pipeline-schema/main/schema.json"
                    },
                    {
                        "fileMatch": ["vercel.json"],
                        "url": "https://openapi.vercel.sh/vercel.json"
                    }
                ]
            }
        })))
    }
}

zed::register_extension!(GithubActionsExtension);
