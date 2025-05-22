/*
    Appellation: impl_builder <module>
    Contrib: @FL03
*/
use super::{Builder, BuilderInner};
use crate::pipes::watch::{WatcherConfig, WatcherState};
use std::sync::Arc;

impl Builder {
    /// Creates a new `Builder` instance with the given configuration and state.
    ///
    /// # Arguments
    ///
    /// * `config` - A `WatcherConfig` instance containing the configuration for the builder.
    /// * `state` - A `WatcherState` instance representing the current state of the builder.
    ///
    /// # Returns
    ///
    /// A new `Builder` instance.
    pub fn new(config: WatcherConfig, state: WatcherState) -> Self {
        Self {
            inner: Arc::new(BuilderInner::new(config, state)),
        }
    }
}

impl Clone for Builder {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl AsRef<Arc<BuilderInner>> for Builder {
    fn as_ref(&self) -> &Arc<BuilderInner> {
        &self.inner
    }
}

impl AsMut<Arc<BuilderInner>> for Builder {
    fn as_mut(&mut self) -> &mut Arc<BuilderInner> {
        &mut self.inner
    }
}

impl core::ops::Deref for Builder {
    type Target = BuilderInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.inner)
    }
}