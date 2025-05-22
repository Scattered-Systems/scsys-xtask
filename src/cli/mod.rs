/*
    Appellation: cli <module>
    Contrib: @FL03
*/
#[doc(inline)]
pub use self::prelude::*;

pub(crate) mod interface;
pub mod opts;

pub mod types {
    #[allow(unused_imports)]
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod targets;

    pub(crate) mod prelude {
        #[doc(inline)]
        pub use super::targets::*;

    }
}

pub(crate) mod prelude {
    #[doc(inline)]
    pub use super::interface::*;
    #[doc(inline)]
    pub use super::opts::prelude::*;
    #[doc(inline)]
    pub use super::types::prelude::*;
}