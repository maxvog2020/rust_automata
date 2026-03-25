use crate::finite::automaton::FiniteAutomaton;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::deterministic::DeterministicAutomaton;

// No complex operations on purpose, convert to NFA first
pub trait DeterministicFiniteAutomaton: DeterministicAutomaton + FiniteAutomaton {
    type CorrespondingNFA: NonDeterministicFiniteAutomaton<State = Self::State, Input = Self::Input>;
    
    fn to_nfa(&self) -> Self::CorrespondingNFA;

    fn minimize(&self) -> Self;
    fn complete(&self) -> Self;
}
