//! Unlabeled reference automata (`Label = ()`).
//!
//! [`SimpleDFA`] and [`SimpleNFA`] are type aliases for
//! [`SimpleLabeledDFA`](crate::labeled::simple::SimpleLabeledDFA) with
//! `Label = ()` and [`SimpleLabeledNFA`](crate::labeled::simple::SimpleLabeledNFA)
//! with `Label = ()`.
//! Accepting states are those listed in constructors; use
//! [`crate::labeled::simple`] for custom label types.

mod dfa;
mod nfa;
mod state;

pub use crate::labeled::simple::SimpleBuildError;
pub use dfa::SimpleDFA;
pub use nfa::SimpleNFA;
pub use state::{SimpleDFAState, SimpleNFAState};
