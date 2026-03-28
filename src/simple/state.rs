//! Re-exports [`SimpleLabeledDFAState`](crate::labeled::simple::SimpleLabeledDFAState) /
//! [`SimpleLabeledNFAState`](crate::labeled::simple::SimpleLabeledNFAState) for the unlabeled aliases.

use crate::labeled::simple::SimpleLabeledDFAState;
use crate::labeled::simple::SimpleLabeledNFAState;

/// [`SimpleDFA`](crate::simple::SimpleDFA) state index.
pub type SimpleDFAState = SimpleLabeledDFAState;
/// [`SimpleNFA`](crate::simple::SimpleNFA) state index.
pub type SimpleNFAState = SimpleLabeledNFAState;
