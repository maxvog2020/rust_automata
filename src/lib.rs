//! `automata_core`: small, explicit automata algorithms in Rust.
//!
//! This crate provides core operations over automata with a clear trait
//! layer split into:
//! - [`general`] (generic automaton concepts), and
//! - [`finite`] (finiteness assumptions + algorithms that rely on them),
//! - [`simple`] (concrete reference implementations: `SimpleDFA` / `SimpleNFA`).
//!
//! The focus is on deterministic and nondeterministic constructions
//! **without ε-transitions in the public trait layer**.
//!
//! # Quick example
//! Build a DFA for the language “even-length words over `{ 'a' }`” and test
//! acceptance.
//!
//! ```rust
//! use automata_core::simple::SimpleDFA;
//! use automata_core::arbitrary::DeterministicAutomaton;
//! use automata_core::finite::DeterministicFiniteAutomaton;
//!
//! let alphabet = ['a'];
//! // 0 = even length, 1 = odd length
//! let edges = [(0usize, 'a', 1usize), (1usize, 'a', 0usize)];
//! let dfa = SimpleDFA::try_new(2, 0, [0], alphabet, edges).unwrap();
//!
//! assert!(dfa.accepts(&[]));
//! assert!(!dfa.accepts(&['a']));
//! assert!(dfa.accepts(&['a', 'a']));
//! ```
//!
//! # Module organization
//! - [`general`]: base `Automaton` trait and determinism/nondeterminism helpers.
//! - [`finite`]: `FiniteAutomaton`, plus algorithms such as `to_dfa`, `complete`,
//!   `minimize`.
//! - [`simple`]: dense reference automata (`SimpleDFA`, `SimpleNFA`).
//!
pub(crate) mod utility;

pub mod arbitrary;
pub mod finite;
pub mod labeled;
pub mod simple;
