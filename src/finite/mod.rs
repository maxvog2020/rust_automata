//! Finite automata in the **unlabeled** API (`Label = ()`).
//!
//! Defines [`FiniteAutomaton`], [`DeterministicFiniteAutomaton`],
//! [`NonDeterministicFiniteAutomaton`], re-exports labeled parsing helpers from
//! [`crate::labeled::finite::parsing`], and hosts algorithms that need to
//! iterate over all states or alphabet symbols (NFA closure ops, `to_dfa`,
//! `complete`, `minimize`, `SimpleDFA::to_matrix`, etc.).
//!
//! For the same operations with arbitrary labels, see [`crate::labeled::finite`].

mod automaton;
mod deterministic;
mod nondeterministic;

pub mod parsing;

pub use automaton::FiniteAutomaton;
pub use deterministic::DeterministicFiniteAutomaton;
pub use nondeterministic::NonDeterministicFiniteAutomaton;
