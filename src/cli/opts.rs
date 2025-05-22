/*
    Appellation: opts <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::build::*;

pub(crate) mod build;

pub(crate) mod prelude {
    #[doc(no_inline)]
    pub use super::Command;
    #[doc(no_inline)]
    pub use super::build::*;
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
    strum::VariantNames,
)]
#[cfg_attr(feature = "clap", derive(clap::Subcommand))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum Command {
    Build(BuildOpts),
}
