/*
    Appellation: pipes <module>
    Contrib: @FL03
*/
//! this module implements the various pipelines for the scsys-xtask library
//! 
//! 

#[doc(inline)]
pub use self::prelude::*;

pub mod watch;


pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::watch::*;
}