use zed::settings::LspSettings;
use zed::serde_json;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct RstExtension {
    cached_binary_path: Option<String>,
}

fn run_cmd(program: &str, args: &[&str], env: &zed::EnvVars) -> Option<String> {
    let output = zed::process::Command::new(program)
        .args(args.iter().copied())
        .envs(env.clone())
        .output()
        .ok()?;
    if output.status != Some(0) {
        return None;
    }
    let stdout = String::from_utf8(output.stdout).ok()?;
    Some(stdout.trim().to_string())
}

impl RstExtension {
    fn find_esbonio(&self, worktree: &zed::Worktree) -> Option<String> {
        if let Some(path) = worktree.which("esbonio") {
            return Some(path);
        }
        let env = worktree.shell_env();
        // worktree.which() may not find esbonio if it's not on the worktree PATH.
        // Ask pipx/uv for their bin directory and construct the full path.
        if worktree.which("pipx").is_some() {
            if let Some(bin_dir) = run_cmd("pipx", &["environment", "--value", "PIPX_BIN_DIR"], &env)
            {
                let path = format!("{}/esbonio", bin_dir);
                if run_cmd(&path, &["--version"], &env).is_some() {
                    return Some(path);
                }
            }
        }
        if worktree.which("uv").is_some() {
            if let Some(bin_dir) = run_cmd("uv", &["tool", "dir", "--bin"], &env) {
                let path = format!("{}/esbonio", bin_dir);
                if run_cmd(&path, &["--version"], &env).is_some() {
                    return Some(path);
                }
            }
        }
        None
    }

    fn ensure_esbonio_installed(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = self.find_esbonio(worktree) {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        let env = worktree.shell_env();

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        if worktree.which("pipx").is_some() {
            run_cmd("pipx", &["install", "esbonio"], &env);
            if self.find_esbonio(worktree).is_none() {
                run_cmd("pipx", &["reinstall", "esbonio"], &env);
            }
        } else if worktree.which("uv").is_some() {
            run_cmd("uv", &["tool", "install", "esbonio"], &env);
            if self.find_esbonio(worktree).is_none() {
                run_cmd("uv", &["tool", "install", "--force", "esbonio"], &env);
            }
        } else {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Failed(
                    "Could not install esbonio: pipx or uv required".to_string(),
                ),
            );
            return Err(
                "esbonio is not installed. Install it with: pipx install esbonio".to_string(),
            );
        }

        if let Some(path) = self.find_esbonio(worktree) {
            self.cached_binary_path = Some(path.clone());
            return Ok(path);
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Failed(
                "esbonio installation failed".to_string(),
            ),
        );
        Err("Failed to install esbonio. Try manually: pipx install esbonio".to_string())
    }

    fn check_for_updates(&self, worktree: &zed::Worktree) {
        let env = worktree.shell_env();
        if worktree.which("pipx").is_some() {
            run_cmd("pipx", &["upgrade", "esbonio"], &env);
        } else if worktree.which("uv").is_some() {
            run_cmd("uv", &["tool", "upgrade", "esbonio"], &env);
        }
    }

    fn esbonio_major_version(&self, binary_path: &str, env: &zed::EnvVars) -> Option<u32> {
        let version_str = run_cmd(binary_path, &["--version"], env)?;
        let major = version_str.split('.').next()?;
        major.parse().ok()
    }
}

impl zed::Extension for RstExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let binary_path = self.ensure_esbonio_installed(language_server_id, worktree)?;
        self.check_for_updates(worktree);

        let env = worktree.shell_env();
        let args = match self.esbonio_major_version(&binary_path, &env) {
            Some(major) if major >= 2 => vec!["server".to_string()],
            Some(_) => vec![],
            None => vec!["server".to_string()],
        };

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::None,
        );

        Ok(zed::Command {
            command: binary_path,
            args,
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("esbonio", worktree)
            .ok()
            .and_then(|s| s.settings);
        Ok(settings)
    }
}

zed::register_extension!(RstExtension);
