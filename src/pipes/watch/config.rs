/*
    Appellation: config <module>
    Contrib: @FL03
*/

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize), serde(default, rename_all = "snake_case"))]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
pub struct WatcherConfig {
    #[cfg_attr(feature = "clap", clap(long, short))]
    pub(crate) target: Option<String>,
    #[cfg_attr(feature = "clap", clap(long, short))]
    pub(crate) output: Option<String>,
    #[cfg_attr(feature = "clap", clap(long, short))]
    pub(crate) input: Option<String>,    
    #[cfg_attr(feature = "clap", arg(long, short, action = clap::ArgAction::SetTrue))]
    pub(crate) verbose: bool,
}

impl WatcherConfig {
    /// Creates a new `WatcherConfig` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the target path.
    pub fn target(&self) -> Option<&String> {
        self.target.as_ref()
    }

    /// Returns the output path.
    pub fn output(&self) -> Option<&String> {
        self.output.as_ref()
    }

    /// Returns the input path.
    pub fn input(&self) -> Option<&String> {
        self.input.as_ref()
    }
}