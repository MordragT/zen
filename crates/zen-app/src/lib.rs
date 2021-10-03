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

mod asset;
mod error;

/// The core trait which holds all the functions that
/// are run during the execution cycle.
pub trait App {
    fn on_init(&mut self, world: &mut World);
    fn on_first(&mut self, world: &mut World);
    fn on_pre_update(&mut self, world: &mut World);
    fn on_update(&mut self, world: &mut World);
    fn on_post_update(&mut self, world: &mut World);
    fn on_last(&mut self, world: &mut World);
}

/// The different stages of the execution cycle
pub enum Stage {
    Init,
    First,
    PreUpdate,
    Update,
    PostUpdate,
    Last,
}
