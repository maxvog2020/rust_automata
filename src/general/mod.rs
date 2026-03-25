//! Core abstractions for automata.
//!
//! This module contains the base [`Automaton`](automaton::Automaton) trait and
//! the determinism/nondeterminism marker traits used by higher-level
//! algorithms.

pub mod automaton;
pub mod deterministic;
pub mod nondeterministic;
