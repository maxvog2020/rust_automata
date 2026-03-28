use crate::labeled::arbitrary::LabeledAutomaton;

/// Base trait for (possibly nondeterministic) automata in the **unlabeled**
/// API (`Label = ()` via [`LabeledAutomaton`]).
///
/// Neither this trait nor [`LabeledAutomaton`] assumes a finite state set or
/// alphabet: [`LabeledAutomaton::states`] and [`LabeledAutomaton::alphabet`]
/// may yield arbitrarily many items. For traits that require collecting states
/// or symbols into sets, see [`crate::finite::FiniteAutomaton`] and
/// [`crate::labeled::finite::FiniteLabeledAutomaton`].
///
/// Provides:
/// - state and input associated types, state and alphabet iterators,
/// - valid / initial predicates, and accepting states via
///   [`is_accepting_state`](Automaton::is_accepting_state) /
///   [`LabeledAutomaton::get_label`](crate::labeled::arbitrary::LabeledAutomaton::get_label).
///
/// Related traits:
/// - [`DeterministicAutomaton`](crate::arbitrary::DeterministicAutomaton): single
///   successor per symbol.
/// - [`NonDeterministicAutomaton`](crate::arbitrary::NonDeterministicAutomaton):
///   sets of successors.
pub trait Automaton: LabeledAutomaton<()> {
    /// Whether `state` is an accepting/final state.
    fn is_accepting_state(&self, state: Self::State) -> bool {
        self.has_label(state)
    }

    /// Iterator over accepting states (those with [`Some`] label), derived from
    /// [`LabeledAutomaton::states`].
    fn accepting_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a {
        self.states().filter(|s| self.is_accepting_state(*s))
    }
}
