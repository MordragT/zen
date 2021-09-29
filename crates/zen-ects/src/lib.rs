#![feature(drain_filter)]
//! Entity Component Trait System
//! Components and Systems are coupled together with rust traits.
//! This trades flexibility with simplicity.
//! Default traits like Physics allow a callback in the lifecycle of World

pub use world::World;

pub mod components;
pub mod error;
pub mod events;
mod world;
