//! Finite automata with finite state sets and finite input alphabets.
//!
//! This module defines:
//! - [`FiniteAutomaton`](automaton::FiniteAutomaton),
//! - [`DeterministicFiniteAutomaton`](deterministic::DeterministicFiniteAutomaton),
//! - [`NonDeterministicFiniteAutomaton`](nondeterministic::NonDeterministicFiniteAutomaton),
//! - Graphviz (`.dot`) support via [`FiniteAutomaton::to_dot`](automaton::FiniteAutomaton::to_dot)
//! - DFA transition matrices via `SimpleDFA::to_matrix` (and similar helpers).

mod automaton;
mod deterministic;
mod nondeterministic;

pub use automaton::FiniteAutomaton;
pub use deterministic::DeterministicFiniteAutomaton;
pub use nondeterministic::NonDeterministicFiniteAutomaton;
