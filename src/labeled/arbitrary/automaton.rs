use std::hash::Hash;

// TODO: docs
pub trait LabeledAutomaton<Label: Hash + Eq + Clone> {
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

    /// Get the label of `state`.
    fn get_label(&self, state: Self::State) -> Option<Label>;
}
