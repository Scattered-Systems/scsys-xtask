/*
    Appellation: watch <module>
    Contrib: @FL03
*/
//! the `watch` module implements the build pipeline for the scsys-xtask library
#[doc(inline)]
pub use self::prelude::*;

pub mod builder;
pub mod config;
pub mod state;

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::builder::*;
    #[doc(inline)]
    pub use super::config::*;
    #[doc(inline)]
    pub use super::state::*;
}