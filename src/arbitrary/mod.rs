//! Core abstractions for automata.
//!
//! This module contains the base [`Automaton`](automaton::Automaton) trait and
//! the determinism/nondeterminism marker traits used by higher-level
//! algorithms.

mod automaton;
mod deterministic;
mod nondeterministic;

pub use automaton::Automaton;
pub use deterministic::DeterministicAutomaton;
pub use nondeterministic::NonDeterministicAutomaton;
