//! Concrete reference automata implementations.
//!
//! The types in this module are intentionally small and explicit:
//! - [`SimpleDFA`](dfa::SimpleDFA): deterministic transition table with
//!   `State × Input -> Option<State>`.
//! - [`SimpleNFA`](nfa::SimpleNFA): nondeterministic transition relation with
//!   `State × Input -> set of states`.

mod dfa;
mod error;
mod nfa;
mod state;

pub use dfa::SimpleDFA;
pub use error::SimpleBuildError;
pub use nfa::SimpleNFA;
pub use state::{SimpleDFAState, SimpleNFAState};
