use crate::{
    arbitrary::automaton::Automaton, labeled::arbitrary::NonDeterministicLabeledAutomaton,
};

/// Marker for nondeterministic automata in the unlabeled API (`Label = ()`).
///
/// Successors are given by [`NonDeterministicLabeledAutomaton::successors`];
/// multiple initial states are allowed via [`NonDeterministicLabeledAutomaton::initial_states`].
pub trait NonDeterministicAutomaton: Automaton + NonDeterministicLabeledAutomaton<()> {}
