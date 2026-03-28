use std::collections::HashSet;

use crate::{arbitrary::Automaton, labeled::finite::FiniteLabeledAutomaton};

/// Finite automata are automata whose state set and input alphabet can be
/// enumerated.
///
/// This trait is used as a *finiteness bound*: algorithms that need to loop
/// over all symbols or all states should require `FiniteAutomaton`.
pub trait FiniteAutomaton: Automaton + FiniteLabeledAutomaton<()> {
    /// Return the automaton's accepting states as a set.
    fn accepting_states_set(&self) -> HashSet<Self::State> {
        self.accepting_states().collect()
    }
}
