//! Labeled automata: states can carry an output **label** type `Label`.
//!
//! - [`arbitrary`]: traits for any (possibly infinite) state/alphabet description.
//! - [`finite`]: finite enumerability, DFA/NFA algorithms, longest-match parsing.
//! - [`simple`]: [`SimpleLabeledDFA`](crate::labeled::simple::SimpleLabeledDFA) and [`SimpleLabeledNFA`](crate::labeled::simple::SimpleLabeledNFA) reference types.
//!
//! A state is **accepting** when [`LabeledAutomaton::get_label`](crate::labeled::arbitrary::LabeledAutomaton::get_label) returns
//! [`Some`]. The unlabeled crate surface ([`crate::arbitrary`], [`crate::simple`])
//! fixes `Label = ()`.
pub mod arbitrary;
pub mod finite;
pub mod simple;
