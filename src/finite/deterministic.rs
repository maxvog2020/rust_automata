use crate::finite::automaton::FiniteAutomaton;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::deterministic::DeterministicAutomaton;

// No complex operations on purpose, convert to NFA first
pub trait DeterministicFiniteAutomaton: DeterministicAutomaton + FiniteAutomaton {
    fn to_nfa<'a>(&'a self) -> impl NonDeterministicFiniteAutomaton + 'a;

    fn complete<'a>(&'a self) -> impl DeterministicFiniteAutomaton + 'a;

    // TODO: fix the lifetime error
    fn minimize<'a>(&'a self) -> impl DeterministicFiniteAutomaton + 'a {
        self.to_nfa().reverse().to_dfa().to_nfa().reverse().to_dfa()
    }
}
