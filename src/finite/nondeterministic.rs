use crate::finite::automaton::FiniteAutomaton;
use crate::finite::deterministic::DeterministicFiniteAutomaton;
use crate::general::nondeterministic::NonDeterministicAutomaton;

pub trait NonDeterministicFiniteAutomaton: NonDeterministicAutomaton + FiniteAutomaton {
    fn to_dfa<'a>(&'a self) -> impl DeterministicFiniteAutomaton + 'a;

    fn union<'a>(&'a self, other: &'a Self) -> impl NonDeterministicFiniteAutomaton + 'a;
    fn difference<'a>(&'a self, other: &'a Self) -> impl NonDeterministicFiniteAutomaton + 'a;
    fn concatenate<'a>(&'a self, other: &'a Self) -> impl NonDeterministicFiniteAutomaton + 'a;
    fn intersection<'a>(&'a self, other: &'a Self) -> impl NonDeterministicFiniteAutomaton + 'a;

    fn star<'a>(&'a self) -> impl NonDeterministicFiniteAutomaton + 'a;
    fn reverse<'a>(&'a self) -> impl NonDeterministicFiniteAutomaton + 'a;

    fn trimmed<'a>(&'a self) -> impl NonDeterministicFiniteAutomaton + 'a;
    fn complement<'a>(&'a self) -> impl NonDeterministicFiniteAutomaton + 'a;

    fn accessible<'a>(&'a self) -> impl NonDeterministicFiniteAutomaton + 'a;
    fn co_accessible(&self) -> impl NonDeterministicFiniteAutomaton;

    fn is_subset_of(&self, other: &Self) -> bool;
    fn is_equivalent_to(&self, other: &Self) -> bool;

    fn is_empty_language(&self) -> bool {
        !self.reachable_states().iter().any(|&s| self.is_accepting_state(s))
    }
}
