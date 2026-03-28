use std::hash::Hash;

use crate::labeled::arbitrary::DeterministicLabeledAutomaton;
use crate::labeled::finite::NonDeterministicFiniteLabeledAutomaton;
use crate::labeled::finite::automaton::FiniteLabeledAutomaton;

/// Deterministic finite automaton with labels (DFA + output on final states).
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

    /// Minimize this DFA (language and label behavior preserved).
    ///
    /// Implementations may use any correct algorithm. The reference type
    /// [`crate::labeled::simple::SimpleLabeledDFA`] uses Hopcroft’s algorithm
    /// after completion, with an initial partition by
    /// [`LabeledAutomaton::get_label`](crate::labeled::arbitrary::LabeledAutomaton::get_label)
    /// (`Option<Label>`), not only accepting vs non-accepting.
    fn minimize(&self) -> Self;
}
