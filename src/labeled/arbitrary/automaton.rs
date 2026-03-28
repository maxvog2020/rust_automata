use std::hash::Hash;

/// Automaton with optional **output labels** on states.
///
/// This trait does **not** assume a finite state set or alphabet: iterators
/// returned by [`states`](LabeledAutomaton::states) and
/// [`alphabet`](LabeledAutomaton::alphabet) are not required to end.
///
/// Semantics are intentionally minimal: there is no built-in “accepting” flag;
/// callers treat states where [`get_label`](LabeledAutomaton::get_label) returns
/// [`Some`] as final / accepting when modeling classical languages. For
/// `Label = ()`, that matches the usual “marked accepting states” encoding.
pub trait LabeledAutomaton<Label: Eq + Clone> {
    /// State type.
    type State: Hash + Eq + Copy;

    /// Input symbol type.
    type Input: Hash + Eq + Clone;

    /// Iterator over states of this automaton (not assumed finite).
    fn states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a;

    /// Iterator over input symbols this automaton uses (not assumed finite).
    fn alphabet<'a>(&'a self) -> impl Iterator<Item = Self::Input> + 'a;

    /// Whether `state` belongs to the automaton.
    fn is_valid_state(&self, state: Self::State) -> bool;

    /// Whether `state` is an initial state.
    fn is_initial_state(&self, state: Self::State) -> bool;

    /// Get the label of `state`.
    fn get_label(&self, state: Self::State) -> Option<Label>;

    /// Iterator over all labels of this automaton (not assumed finite).
    fn labels<'a>(&'a self) -> impl Iterator<Item = Label> + 'a
    where
        Label: 'a,
    {
        self.states().flat_map(|s| self.get_label(s))
    }
}
