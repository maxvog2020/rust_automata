//! State indices for [`super::SimpleLabeledDFA`] and [`super::SimpleLabeledNFA`].

/// State id in a [`super::SimpleLabeledDFA`] (dense `0..n`).
pub type SimpleLabeledDFAState = usize;
/// State id in a [`super::SimpleLabeledNFA`] (dense `0..n`).
pub type SimpleLabeledNFAState = usize;
