use crate::finite::automaton::FiniteAutomaton;
use crate::finite::deterministic::DeterministicFiniteAutomaton;
use crate::general::nondeterministic::NonDeterministicAutomaton;

pub trait NonDeterministicFiniteAutomaton: NonDeterministicAutomaton + FiniteAutomaton {
    fn to_dfa(&self) -> impl DeterministicFiniteAutomaton;

    fn union(&self, other: &Self) -> impl NonDeterministicFiniteAutomaton;
    fn difference(&self, other: &Self) -> impl NonDeterministicFiniteAutomaton;
    fn concatenate(&self, other: &Self) -> impl NonDeterministicFiniteAutomaton;
    fn intersection(&self, other: &Self) -> impl NonDeterministicFiniteAutomaton;

    fn star(&self) -> impl NonDeterministicFiniteAutomaton;
    fn reverse(&self) -> impl NonDeterministicFiniteAutomaton;

    fn trimmed(&self) -> impl NonDeterministicFiniteAutomaton;
    fn complement(&self) -> impl NonDeterministicFiniteAutomaton;

    fn accessible(&self) -> impl NonDeterministicFiniteAutomaton;
    fn co_accessible(&self) -> impl NonDeterministicFiniteAutomaton;

    fn is_empty_language(&self) -> bool {
        todo!("NonDeterministicFiniteAutomaton::is_empty_language")
    }

    fn equivalent(&self, _other: &Self) -> bool {
        todo!("NonDeterministicFiniteAutomaton::equivalent")
    }
}
