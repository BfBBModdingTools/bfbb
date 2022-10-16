//! A library for creating tools and mods for *SpongeBob SquarePants: Battle for Bikini Bottom (2003)*.
//!
// TODO: Add docs here that introduce [`GameInterface`] as an entry-point to the library and give examples of how
//       to get one.
//!
//! # Feature flags
//! Some functionality is gated behind feature flags to allow you to pick and choose what functionality you need for your project.
//!
//! `serde`: Implements [`serde`](https://docs.rs/serde)'s `Serialize`/`Deserialize` for the core data types.
//!
//! `game-interface`: Enables the [`game_interface`] module.
//!
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub use level::Level;
pub use spatula::Spatula;

pub use strum::{EnumCount, IntoEnumIterator};

#[cfg(feature = "game-interface")]
pub mod game_interface;

mod endian;
pub mod game_state;
mod level;
mod spatula;
