//! # Logical Interactions
//!
//! # Library Layout
//! ## Referee
//! Contains an implementation of the [`referee::Referee`] which is an arbiter of a Maze`.`com
//! game.
//!
//! ## Player
//! Contains a wrapper around a `PlayerApi` and a `PrivatePlayerInfo` for convenience and coupling
//! the communication aspect of a player and the information attached to a player. This
//! [`player::Player`] also contains the safe-guarding of the referee from misbehaving players.
//!
//! ## Observer
//! Contains the [`observer::Observer`] trait which describes a "GameListener" that the Referee
//! updates with the current state.
//!
//! ### Json
//! Contains the data definitions for integration tests of the [`referee::Referee`].

/// Contains the Observer trait and a GUI implementation.
pub mod observer;

/// Contains all the data types and functionalities used for interfacing with Players and handling timeouts
pub mod player;

/// Contains the `Referee` implementation
pub mod referee;

/// Contains all the JSON definitions needed for the Referee integration tests.
pub mod json;
