use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct RstExtension;

impl RstExtension {
    fn language_server_binary_path(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("esbonio") {
            return Ok(path);
        }

        Err(format!("esbonio not installed"))

        // TODO: auto-download/install?
    }
}

impl zed::Extension for RstExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(RstExtension);
