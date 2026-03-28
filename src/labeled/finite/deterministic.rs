use std::hash::Hash;

use crate::labeled::arbitrary::DeterministicLabeledAutomaton;
use crate::labeled::finite::automaton::FiniteLabeledAutomaton;
use crate::labeled::finite::NonDeterministicFiniteLabeledAutomaton;

// TODO: docs
pub trait DeterministicFiniteLabeledAutomaton<Label: Hash + Eq + Clone>: DeterministicLabeledAutomaton<Label> + FiniteLabeledAutomaton<Label> {
    type CorrespondingNFA: NonDeterministicFiniteLabeledAutomaton<
            Label,
            State = Self::State,
            Input = Self::Input,
            CorrespondingDFA = Self,
        >;

    // TODO: docs
    fn to_nfa(&self) -> Self::CorrespondingNFA;

    // TODO: docs
    fn complete(&self) -> Self;

    // TODO: docs
    fn minimize(&self) -> Self;
}
