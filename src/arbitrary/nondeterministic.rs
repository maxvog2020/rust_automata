use crate::{arbitrary::automaton::Automaton, labeled::arbitrary::NonDeterministicLabeledAutomaton};

// TODO: docs
pub trait NonDeterministicAutomaton: Automaton + NonDeterministicLabeledAutomaton<()> {}
