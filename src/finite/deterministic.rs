use crate::finite::automaton::FiniteAutomaton;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::deterministic::DeterministicAutomaton;

// No complex operations on purpose, convert to NFA first
pub trait DeterministicFiniteAutomaton: DeterministicAutomaton + FiniteAutomaton {
    fn to_nfa(&self) -> impl NonDeterministicFiniteAutomaton;

    fn minimize(&self) -> impl DeterministicFiniteAutomaton;
    fn complete(&self) -> impl DeterministicFiniteAutomaton;
}
