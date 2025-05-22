/*
    Appellation: scsys-xtask <library>
    Contrib: @FL03
*/
//! # scsys-xtask
//! 
//! The `scsys-xtask` library provides a common set of primitives and utilities for building 
//! and managing projects in the `scsys` ecosystem.
#![crate_name = "scsys_xtask"]
#![crate_type = "lib"]

#[doc(inline)]
pub use self::error::*;

#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod pipes;

pub mod prelude {
    #[cfg(feature = "cli")]
    pub use crate::cli::prelude::*;
    pub use crate::error::*;
    pub use crate::pipes::prelude::*;

}