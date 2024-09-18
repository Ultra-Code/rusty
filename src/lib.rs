//! library implementation of `mgrep` and `guessing_game`
#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![feature(vec_push_within_capacity)]
pub use mgrep::Case;
pub mod guessing_game;
pub mod mgrep;
pub mod threadpool;
pub mod webserver;
