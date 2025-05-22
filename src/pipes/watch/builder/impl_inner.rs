/*
    Appellation: impl_inner <module>
    Contrib: @FL03
*/
use super::BuilderInner;
use crate::pipes::watch::{WatcherConfig, WatcherState};

impl BuilderInner {
    /// Creates a new `BuilderInner` instance with the given configuration and state.
    ///
    /// # Arguments
    ///
    /// * `config` - A `WatcherConfig` instance containing the configuration for the builder.
    /// * `state` - A `WatcherState` instance representing the current state of the builder.
    ///
    /// # Returns
    ///
    /// A new `BuilderInner` instance.
    pub fn new(config: WatcherConfig, state: WatcherState) -> Self {
        Self { config, state }
    }
    /// returns a new instance of the builder initialized with the given configuration
    pub fn from_config(config: WatcherConfig) -> Self {
        Self {
            config,
            state: WatcherState::default(),
        }
    }
    /// returns a new instance of the builder initialized with the given state
    pub fn from_state(state: WatcherState) -> Self {
        Self {
            config: WatcherConfig::default(),
            state,
        }
    }
    /// returns an immutable reference to the current configuration of the pipeline
    pub const fn config(&self) -> &WatcherConfig {
        &self.config
    }
    /// returns a mutable reference to the current configuration of the pipeline
    pub fn config_mut(&mut self) -> &mut WatcherConfig {
        &mut self.config
    }
    /// returns an immutable reference to the pipelines current state
    pub const fn state(&self) -> &WatcherState {
        &self.state
    }
    /// returns a mutable reference to the pipelines current state
    pub fn state_mut(&mut self) -> &mut WatcherState {
        &mut self.state
    }
    /// set the configuration of the pipeline and return a mutable reference to the instance
    pub fn set_config(&mut self, config: WatcherConfig) -> &mut Self {
        self.config = config;
        self
    }
    /// set the state of the pipeline and return a mutable reference to the instance
    pub fn set_state(&mut self, state: WatcherState) -> &mut Self {
        self.state = state;
        self
    }
    /// consumes the current instance to create another with the given configuration
    pub fn with_config(self, config: WatcherConfig) -> Self {
        Self {
            config,
            ..self
        }
    }
    /// consumes the current instance to create another with the given state
    pub fn with_state(self, state: WatcherState) -> Self {
        Self { state, ..self }
    }
}

impl Default for BuilderInner {
    fn default() -> Self {
        Self {
            config: WatcherConfig::default(),
            state: WatcherState::default(),
        }
    }
}