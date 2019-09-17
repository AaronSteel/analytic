//! Provides a layer of abstraction over certain mathematical constructs
//! through trait definitions and concrete implementations.

#![feature(drain_filter)]

pub mod histogram;
pub mod interval;
pub mod iter;
pub mod partition;
pub mod sample;
pub mod search;
pub mod set;
pub mod stats;
pub mod traits;
