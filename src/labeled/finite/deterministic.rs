use std::hash::Hash;

use crate::labeled::arbitrary::DeterministicLabeledAutomaton;
use crate::labeled::finite::NonDeterministicFiniteLabeledAutomaton;
use crate::labeled::finite::automaton::FiniteLabeledAutomaton;

// TODO: docs
pub trait DeterministicFiniteLabeledAutomaton<Label: Hash + Eq + Clone>:
    DeterministicLabeledAutomaton<Label> + FiniteLabeledAutomaton<Label>
{
    type CorrespondingNFA: NonDeterministicFiniteLabeledAutomaton<
            Label,
            State = Self::State,
            Input = Self::Input,
            CorrespondingDFA = Self,
        >;

    /// Convert this DFA into an equivalent NFA with the same state set and step
    /// semantics: for any word `w`, the run from the initial state ends in state
    /// `q` in the DFA iff it ends in `q` in the NFA. Labels are preserved.
    fn to_nfa(&self) -> Self::CorrespondingNFA;

    /// Make the DFA *total* by adding a sink/trap state for missing
    /// transitions, setting the label of the sink state to `None`.
    fn complete(&self) -> Self;

    /// Minimize this DFA.
    ///
    /// The concrete implementation is free to choose an algorithm; the
    /// default implementation uses Hopcroft's approach.
    fn minimize(&self) -> Self;
}
