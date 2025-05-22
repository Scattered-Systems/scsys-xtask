/*
    Appellation: state <module>
    Contrib: @FL03
*/
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WatcherState {
    pub(crate) status: Option<String>,
}
