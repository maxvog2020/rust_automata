use std::hash::Hash;

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
pub trait Automaton {
    /// State type.
    type State: Hash + Eq + Copy;

    /// Input symbol type.
    type Input: Hash + Eq + Clone;

    /// Iterate over all states of the automaton.
    fn states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a;

    /// Iterate over the input symbols of the automaton.
    fn alphabet<'a>(&'a self) -> impl Iterator<Item = Self::Input> + 'a;

    /// Whether `state` belongs to the automaton.
    fn is_valid_state(&self, state: Self::State) -> bool;

    /// Whether `state` is an initial state.
    fn is_initial_state(&self, state: Self::State) -> bool;

    /// Whether `state` is an accepting/final state.
    fn is_accepting_state(&self, state: Self::State) -> bool;
}
