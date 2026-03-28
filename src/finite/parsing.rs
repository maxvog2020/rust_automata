//! Longest-match DFA parsing (unlabeled entry point).
//!
//! Re-exports [`parse_by_longest_match`] and [`ParseResult`] from
//! [`crate::labeled::finite::parsing`]. Parsing works for any
//! [`crate::labeled::finite::DeterministicFiniteLabeledAutomaton`]; unlabeled
//! DFAs use `Label = ()`.

pub use crate::labeled::finite::parsing::ParseResult;
pub use crate::labeled::finite::parsing::parse_by_longest_match;
