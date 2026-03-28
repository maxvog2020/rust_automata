use crate::arbitrary::automaton::Automaton;
use crate::arbitrary::deterministic::DeterministicAutomaton;

/// Nondeterministic automaton semantics.
///
/// Implementors define:
/// - a set of initial states via [`NonDeterministicAutomaton::initial_states`]
/// - a successor relation via [`NonDeterministicAutomaton::successors`]
///   (state + input symbol -> zero or more next states).
pub trait NonDeterministicAutomaton: Automaton {
    /// Iterator over initial states.
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a;

    /// Successors of `state` under `input`.
    fn successors<'a>(
        &'a self,
        state: Self::State,
        input: &Self::Input,
    ) -> impl Iterator<Item = Self::State> + 'a;
}

impl<T: DeterministicAutomaton> NonDeterministicAutomaton for T {
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a {
        core::iter::once(self.initial_state())
    }

    fn successors<'a>(
        &'a self,
        state: Self::State,
        input: &Self::Input,
    ) -> impl Iterator<Item = Self::State> + 'a {
        self.transition(state, input).into_iter()
    }
}
