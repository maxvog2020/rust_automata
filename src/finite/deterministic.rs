use crate::finite::automaton::FiniteAutomaton;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::deterministic::DeterministicAutomaton;

// No complex operations on purpose, convert to NFA first
pub trait DeterministicFiniteAutomaton: DeterministicAutomaton + FiniteAutomaton {
    fn to_nfa<'a>(&'a self) -> impl NonDeterministicFiniteAutomaton + 'a;

    fn minimize<'a>(&'a self) -> impl DeterministicFiniteAutomaton + 'a;
    fn complete<'a>(&'a self) -> impl DeterministicFiniteAutomaton + 'a;
}
