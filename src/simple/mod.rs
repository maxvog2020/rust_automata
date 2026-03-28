//! Concrete reference automata implementations.
//!
//! The types in this module are intentionally small and explicit:
//! - [`SimpleDFA`](dfa::SimpleDFA): deterministic transition table with
//!   `State × Input -> Option<State>`.
//! - [`SimpleNFA`](nfa::SimpleNFA): nondeterministic transition relation with
//!   `State × Input -> set of states`.

mod dfa;
mod nfa;
mod state;

pub use crate::labeled::simple::SimpleBuildError;
pub use dfa::SimpleDFA;
pub use nfa::SimpleNFA;
pub use state::{SimpleDFAState, SimpleNFAState};
