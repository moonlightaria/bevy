use crate::{Flag, Prepare, PreparedCommand};
use argh::FromArgs;
use xshell::cmd;

/// Check code formatting.
#[derive(FromArgs, Default)]
#[argh(subcommand, name = "format")]
pub struct FormatCommand {}

impl Prepare for FormatCommand {
    fn prepare<'a>(&self, sh: &'a xshell::Shell, flags: Flag) -> Vec<PreparedCommand<'a>> {
        let quiet = flags
            .contains(Flag::QUIET)
            .then_some("--quiet")
            .unwrap_or_default();

        vec![PreparedCommand::new::<Self>(
            cmd!(sh, "cargo fmt --all -- --check {quiet}"),
            "Please run 'cargo fmt --all' to format your code.",
        )]
    }
}
