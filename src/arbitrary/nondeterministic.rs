use crate::arbitrary::automaton::Automaton;

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
