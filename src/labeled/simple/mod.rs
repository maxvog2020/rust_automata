//! Dense **labeled** reference automata (`usize` states, `char` alphabet).
//!
//! - [`SimpleLabeledDFA`]: one transition per `(state, symbol)` when defined;
//!   labels live in a separate map (states without an entry are non-final).
//! - [`SimpleLabeledNFA`]: `HashSet` successors per `(state, symbol)`; multiple
//!   initial states allowed.
//!
//! The crate root [`SimpleDFA`](crate::simple::SimpleDFA) /
//! [`SimpleNFA`](crate::simple::SimpleNFA) are type aliases with `Label = ()`.

mod dfa;
mod error;
mod nfa;
mod state;

pub use dfa::SimpleLabeledDFA;
pub use error::SimpleBuildError;
pub use nfa::SimpleLabeledNFA;
pub use state::{SimpleLabeledDFAState, SimpleLabeledNFAState};
