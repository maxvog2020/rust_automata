//! `automata_core`: small, explicit automata algorithms in Rust.
//!
//! The crate is layered as follows:
//!
//! - **[`labeled::arbitrary`]**, **[`arbitrary`]**: core traits (`LabeledAutomaton`,
//!   [`DeterministicLabeledAutomaton`](crate::labeled::arbitrary::DeterministicLabeledAutomaton),
//!   …). Final states carry an optional label; `Label = ()` gives the usual
//!   accepting-state encoding. **No assumption** that state sets or alphabets
//!   are finite.
//! - **[`labeled::finite`]**, **[`finite`]**: add enumerable state sets and
//!   alphabets for algorithms (NFA closure ops, `to_dfa`, `complete`, `minimize`,
//!   parsing, …).
//! - **[`simple`]**: dense reference types [`SimpleDFA`](crate::simple::SimpleDFA) and [`SimpleNFA`](crate::simple::SimpleNFA)
//!   (aliases for [`labeled::simple::SimpleLabeledDFA`]`<()` and
//!   [`labeled::simple::SimpleLabeledNFA`]`<()`).
//!
//! There are **no ε-transitions** in the public trait layer.
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
//! - [`labeled::arbitrary`], [`labeled::finite`], [`labeled::simple`]: labeled
//!   traits and `SimpleLabeledDFA` / `SimpleLabeledNFA`.
//! - [`arbitrary`], [`finite`], [`simple`]: thin unlabeled entry points.
//!
pub(crate) mod utility;

pub mod arbitrary;
pub mod finite;
pub mod labeled;
pub mod simple;
