use std::collections::HashSet;

use crate::general::Automaton;

/// Finite automata are automata whose state set and input alphabet can be
/// enumerated.
///
/// This trait is used as a *finiteness bound*: algorithms that need to loop
/// over all symbols or all states should require `FiniteAutomaton`.
pub trait FiniteAutomaton: Automaton {
    /// Return the automaton's alphabet as a set.
    fn alphabet_set(&self) -> HashSet<Self::Input> {
        self.alphabet().collect()
    }

    /// Return the automaton's states as a set.
    fn states_set(&self) -> HashSet<Self::State> {
        self.states().collect()
    }

    /// Return the automaton's accepting states as a set.
    fn accepting_states_set(&self) -> HashSet<Self::State> {
        self.accepting_states().collect()
    }
}
