use std::hash::Hash;

use crate::labeled::arbitrary::NonDeterministicLabeledAutomaton;
use crate::labeled::finite::automaton::FiniteLabeledAutomaton;
use crate::labeled::finite::deterministic::DeterministicFiniteLabeledAutomaton;

// TODO: docs
pub trait NonDeterministicFiniteLabeledAutomaton<Label: Hash + Eq + Clone>: NonDeterministicLabeledAutomaton<Label> + FiniteLabeledAutomaton<Label> {
    /// Deterministic representation obtained by determinization.
    type CorrespondingDFA: DeterministicFiniteLabeledAutomaton<
            Label,
            State = Self::State,
            Input = Self::Input,
            CorrespondingNFA = Self,
        >;

    /// Determinize this NFA into a DFA (subset construction).
    fn to_dfa_by(&self, combine: impl Fn(Label, Label) -> Label) -> Self::CorrespondingDFA;

    // TODO: docs
    fn union(&self, other: &Self) -> Self;
}

