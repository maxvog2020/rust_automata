//! Finite automata with finite state sets and finite input alphabets.
//!
//! This module defines:
//! - [`FiniteAutomaton`](automaton::FiniteAutomaton),
//! - [`DeterministicFiniteAutomaton`](deterministic::DeterministicFiniteAutomaton),
//! - [`NonDeterministicFiniteAutomaton`](nondeterministic::NonDeterministicFiniteAutomaton),
//! plus Graphviz (`.dot`) support via [`FiniteAutomaton::to_dot`](automaton::FiniteAutomaton::to_dot)
//! and DFA transition matrices via [`DeterministicFiniteAutomaton::to_matrix`](deterministic::DeterministicFiniteAutomaton::to_matrix).

mod automaton;
mod deterministic;
mod nondeterministic;

pub use automaton::FiniteAutomaton;
pub use deterministic::DeterministicFiniteAutomaton;
pub use nondeterministic::NonDeterministicFiniteAutomaton;
