use super::*;
use crate::gallery::command::CommandInfo;

pub struct Tools {
    pub git: CommandInfo,
    pub cargo_dist: CommandInfo,
    pub shellcheck: Option<CommandInfo>,
    pub psanalyzer: Option<CommandInfo>,
    pub homebrew: Option<CommandInfo>,
}

impl Tools {
    fn new() -> Self {
        eprintln!("getting tools...");
        let git = CommandInfo::new("git", None).expect("git isn't installed");

        // If OVERRIDE_* is set, prefer that over the version that cargo built for us,
        // this lets us test our shippable builds.
        let cargo_dist_path = std::env::var(ENV_RUNTIME_CARGO_DIST_BIN)
            .unwrap_or_else(|_| STATIC_CARGO_DIST_BIN.to_owned());
        let cargo_dist = CommandInfo::new("cargo-dist", Some(&cargo_dist_path))
            .expect("cargo-dist isn't built!?");
        cargo_dist
            .version()
            .expect("couldn't parse cargo-dist version!?");
        let shellcheck = CommandInfo::new("shellcheck", None);
        let psanalyzer = CommandInfo::new_powershell_command("Invoke-ScriptAnalyzer");
        let homebrew = CommandInfo::new("brew", None);

        Self {
            git,
            cargo_dist,
            shellcheck,
            psanalyzer,
            homebrew,
        }
    }
}

impl ToolsImpl for Tools {
    fn git(&self) -> &CommandInfo {
        &self.git
    }
}
impl Default for Tools {
    fn default() -> Self {
        Self::new()
    }
}
