//! Adapters for parsing [`clap`] arguments to various types.

mod camino;
mod humantime;

pub use self::humantime::DurationValueParser;
