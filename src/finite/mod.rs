//! Finite automata with finite state sets and finite input alphabets.
//!
//! This module defines:
//! - [`FiniteAutomaton`](automaton::FiniteAutomaton),
//! - [`DeterministicFiniteAutomaton`](deterministic::DeterministicFiniteAutomaton),
//! - [`NonDeterministicFiniteAutomaton`](nondeterministic::NonDeterministicFiniteAutomaton),
//! - DFA transition matrices via `SimpleDFA::to_matrix` (and similar helpers).
//! - Lexer-style longest-match parsing for DFAs via [`parsing::parse_by_longest_match`](parsing::parse_by_longest_match).

mod automaton;
mod deterministic;
mod nondeterministic;

pub mod parsing;

pub use automaton::FiniteAutomaton;
pub use deterministic::DeterministicFiniteAutomaton;
pub use nondeterministic::NonDeterministicFiniteAutomaton;
