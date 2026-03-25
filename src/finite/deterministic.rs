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

    /// Minimize this DFA.
    ///
    /// The concrete implementation is free to choose an algorithm; the
    /// `SimpleDFA` implementation uses Brzozowski's approach (via reverse +
    /// determinization).
    fn minimize(&self) -> Self;

    /// Make the DFA *total* by adding a sink/trap state for missing
    /// transitions.
    fn complete(&self) -> Self;

    /// Convert many DFAs to NFAs.
    ///
    /// Returns an iterator of NFAs corresponding to the input DFAs.
    fn to_nfa_all(automata: impl IntoIterator<Item = Self>) -> impl IntoIterator<Item = Self::CorrespondingNFA> 
        where Self: Sized 
    {
        automata.into_iter().map(|a| a.to_nfa())
    }
}
