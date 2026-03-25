use crate::finite::automaton::FiniteAutomaton;
use crate::finite::deterministic::DeterministicFiniteAutomaton;
use crate::general::NonDeterministicAutomaton;

pub trait NonDeterministicFiniteAutomaton: NonDeterministicAutomaton + FiniteAutomaton {
    type CorrespondingDFA: DeterministicFiniteAutomaton<State = Self::State, Input = Self::Input>;

    fn to_dfa(&self) -> Self::CorrespondingDFA;

    fn union(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    fn concatenate(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;

    fn star(&self) -> Self;
    fn reverse(&self) -> Self;

    fn trimmed(&self) -> Self;
    fn complement(&self) -> Self;

    fn accessible(&self) -> Self;
    fn co_accessible(&self) -> Self;

    fn is_subset_of(&self, other: &Self) -> bool;
    fn is_equivalent_to(&self, other: &Self) -> bool;

    fn is_empty_language(&self) -> bool {
        !self.reachable_states().iter().any(|&s| self.is_accepting_state(s))
    }
}
