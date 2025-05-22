/*
    Appellation: builder <module>
    Contrib: @FL03
*/

mod impl_builder;
mod impl_inner;

use super::{WatcherConfig, WatcherState};
use std::sync::Arc;

pub struct Builder {
    pub(crate) inner: Arc<BuilderInner>,
}

#[derive(Clone, Debug)]
pub struct BuilderInner {
    pub(crate) config: WatcherConfig,
    pub(crate) state: WatcherState,
}
