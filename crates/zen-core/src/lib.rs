//! This crate holds the main [App] trait which is
//! required to run.
//! Aswell as other helpers like an asset loader trait.
//! The App trait holds different functions which are
//! run during different stages of the application execution cycle.
//! Init is run only once before all other stages.
//! The other cycles are self explanatory.

pub use asset::*;
pub use error::Error;
use hecs::World;
pub use transform::*;

mod asset;
mod error;
mod transform;

pub type EventQueue<T> = Vec<T>;
pub type TimeDelta = std::time::Duration;

pub struct Resource<T> {
    inner: T,
}

impl<T> Resource<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    pub fn replace(&mut self, inner: T) {
        self.inner = inner;
    }
}
