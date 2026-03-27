use crate::finite::automaton::FiniteAutomaton;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::DeterministicAutomaton;

/// Deterministic finite automata (DFA) operations over finite state sets.
///
/// This trait is the DFA counterpart of
/// [`finite::nondeterministic::NonDeterministicFiniteAutomaton`].
/// Implementations provide determinism-specific operations such as word
/// acceptance (via [`DeterministicAutomaton::accepts`]), completion, and
/// minimization.
pub trait DeterministicFiniteAutomaton: DeterministicAutomaton + FiniteAutomaton {
    type CorrespondingNFA: NonDeterministicFiniteAutomaton<State = Self::State, Input = Self::Input, CorrespondingDFA = Self>;
    
    /// Convert this DFA into an equivalent NFA.
    fn to_nfa(&self) -> Self::CorrespondingNFA;

    /// Make the DFA *total* by adding a sink/trap state for missing
    /// transitions.
    fn complete(&self) -> Self;

    /// Minimize this DFA.
    ///
    /// The concrete implementation is free to choose an algorithm; the
    /// default implementation uses Brzozowski's approach (via reverse +
    /// determinization).
    fn minimize(&self) -> Self 
        where Self: Sized 
    {
        self.to_nfa().to_minimized_dfa()
    }
}
