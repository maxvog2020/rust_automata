//! Labeled automaton traits: **no** assumption that state sets or alphabets are
//! finite (iterators may not terminate).
//!
//! - [`LabeledAutomaton`]: states, alphabet, initial states, optional labels.
//! - [`DeterministicLabeledAutomaton`]: unique initial state and transition function.
//! - [`NonDeterministicLabeledAutomaton`]: multiple initials and successor sets.

mod automaton;
mod deterministic;
mod nondeterministic;

pub use automaton::LabeledAutomaton;
pub use deterministic::DeterministicLabeledAutomaton;
pub use nondeterministic::NonDeterministicLabeledAutomaton;
