use crate::labeled::arbitrary::LabeledAutomaton;

/// Base trait for (possibly nondeterministic) automata.
///
/// The trait defines:
/// - a state type (`State`) and a method to iterate over states,
/// - an input type (`Input`) and a method to iterate over alphabet symbols,
/// - predicates for valid/initial/accepting states.
///
/// Note: this `general` layer does not assume finiteness. The `finite`
/// layer adds the finiteness bound where algorithms require it.
///
/// Higher-level traits build on this:
/// - [`general::deterministic::DeterministicAutomaton`] for deterministic step
///   semantics and word acceptance.
/// - [`general::nondeterministic::NonDeterministicAutomaton`] for successor
///   sets.
pub trait Automaton: LabeledAutomaton<()> {
    /// Whether `state` is an accepting/final state.
    fn is_accepting_state(&self, state: Self::State) -> bool {
        self.get_label(state).is_some()
    }

    /// Iterate over all accepting states of the automaton.
    fn accepting_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a {
        self.states().filter(|s| self.is_accepting_state(*s))
    }
}
