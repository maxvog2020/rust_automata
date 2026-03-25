use crate::finite::automaton::FiniteAutomaton;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::deterministic::DeterministicAutomaton;

pub trait DeterministicFiniteAutomaton: DeterministicAutomaton + FiniteAutomaton {
    fn to_nfa(&self) -> impl NonDeterministicFiniteAutomaton;

    fn minimize(&self) -> impl DeterministicFiniteAutomaton;

    // TODO: default bodies (return-position `impl Trait` cannot use `todo!()` in trait defaults).
    fn accessible(&self) -> impl DeterministicFiniteAutomaton;

    fn co_accessible(&self) -> impl DeterministicFiniteAutomaton;

    fn trimmed(&self) -> impl DeterministicFiniteAutomaton;

    fn complete(&self) -> impl DeterministicFiniteAutomaton;

    fn is_empty_language(&self) -> bool {
        todo!("DeterministicFiniteAutomaton::is_empty_language")
    }

    fn equivalent(&self, _other: &Self) -> bool {
        todo!("DeterministicFiniteAutomaton::equivalent")
    }
}
