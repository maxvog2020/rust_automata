//! Unlabeled automata traits (`Label = ()`): same trait shapes as
//! [`crate::labeled::arbitrary`]. State sets and alphabets are **not** assumed
//! finite.
//!
//! A state is **accepting** iff [`LabeledAutomaton::get_label`](crate::labeled::arbitrary::LabeledAutomaton::get_label)
//! returns `Some(())`. Use [`crate::labeled`] when you need structured output
//! on final states (tokens, priorities, etc.).

mod automaton;
mod deterministic;
mod nondeterministic;

pub use automaton::Automaton;
pub use deterministic::DeterministicAutomaton;
pub use nondeterministic::NonDeterministicAutomaton;
