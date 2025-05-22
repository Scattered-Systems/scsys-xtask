use super::Command;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
#[cfg_attr(feature = "clap", clap(about, author, long_about = None, version))]
#[command(arg_required_else_help(true), allow_missing_positional(true))]
pub struct Cli {
    #[cfg_attr(feature = "clap", clap(subcommand))]
    pub command: Option<Command>,
    #[cfg_attr(feature = "clap", clap(long, short = 'C', default_value_t = String::from("xtask.toml")))]
    pub config: String,
    #[cfg_attr(feature = "clap", arg(action = clap::ArgAction::SetTrue, long, short))]
    pub release: bool,
    #[cfg_attr(feature = "clap", arg(action = clap::ArgAction::SetTrue, long, short))]
    pub update: bool,
    #[cfg_attr(feature = "clap", arg(action = clap::ArgAction::SetTrue, long, short))]
    pub verbose: bool,
}

impl Cli {
    #[cfg(feature = "clap")]
    pub fn parse() -> Self {
        clap::Parser::parse()
    }

    pub fn command(&self) -> Option<&Command> {
        self.command.as_ref()
    }

    pub fn config(&self) -> &str {
        &self.config
    }

    pub fn release(&self) -> bool {
        self.release
    }

    pub fn update(&self) -> bool {
        self.update
    }

    pub fn verbose(&self) -> bool {
        self.verbose
    }
}
