use crate::arbitrary::DeterministicAutomaton;
use crate::finite::automaton::FiniteAutomaton;
use crate::labeled::finite::DeterministicFiniteLabeledAutomaton;

/// Deterministic finite automata (DFA) operations over finite state sets.
///
/// This trait is the DFA counterpart of
/// [`NonDeterministicFiniteAutomaton`](crate::finite::NonDeterministicFiniteAutomaton).
/// Implementations provide determinism-specific operations such as word
/// acceptance (via [`DeterministicAutomaton::accepts`]), completion, and
/// minimization.
pub trait DeterministicFiniteAutomaton:
    DeterministicAutomaton + FiniteAutomaton + DeterministicFiniteLabeledAutomaton<()>
{
}
