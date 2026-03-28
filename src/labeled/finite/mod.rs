//! Finite labeled automata: enumerable states and alphabet.
//!
//! Comprises [`FiniteLabeledAutomaton`], [`DeterministicFiniteLabeledAutomaton`],
//! [`NonDeterministicFiniteLabeledAutomaton`], and [`parsing`] for DFA
//! longest-match lexing.

mod automaton;
mod deterministic;
mod nondeterministic;

pub mod parsing;

pub use automaton::FiniteLabeledAutomaton;
pub use deterministic::DeterministicFiniteLabeledAutomaton;
pub use nondeterministic::NonDeterministicFiniteLabeledAutomaton;
