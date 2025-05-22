/*
    Appellation: build <module>
    Contrib: @FL03
*/

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "kebab-case")
)]
pub struct BuildOpts {
    #[cfg_attr(feature = "clap", clap(subcommand))]
    pub args: Option<BuildCmd>,
    #[cfg_attr(feature = "clap", clap(long, short))]
    pub platform: Option<TargetPlatforms>,
    #[cfg_attr(feature = "clap", clap(long, short))]
    pub target: Option<String>,
}

#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, smart_default::SmartDefault, strum::EnumIs,
)]
#[cfg_attr(feature = "clap", derive(clap::Subcommand))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "kebab-case")
)]
#[strum(serialize_all = "lowercase")]
pub enum BuildCmd {
    #[default]
    A,
}

#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    scsys::VariantConstructors,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "kebab-case")
)]
#[strum(serialize_all = "lowercase")]
pub enum TargetPlatforms {
    Wasm,
}
