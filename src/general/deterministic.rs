use crate::general::automaton::Automaton;

/// Deterministic automaton semantics.
///
/// Implementors provide a single `transition` function. This trait then
/// supplies [`accepts`] as a word-level convenience.
pub trait DeterministicAutomaton: Automaton {
    /// The unique initial state.
    fn initial_state(&self) -> Self::State;

    /// The deterministic step function.
    fn transition(&self, state: Self::State, input: &Self::Input) -> Option<Self::State>;

    /// Run the automaton on `word` **starting from** `state` (not necessarily
    /// [`initial_state`](DeterministicAutomaton::initial_state)).
    ///
    /// Applies [`transition`] once per symbol in order. Returns `Some(q)` if
    /// the entire word is read (every step succeeded); otherwise `None`
    /// (an undefined transition or invalid configuration).
    fn run_from(&self, state: Self::State, word: &[Self::Input]) -> Option<Self::State> {
        let mut current_state = state;
        for input in word {
            current_state = self.transition(current_state, input)?;
        }
        Some(current_state)
    }

    /// Check whether the automaton accepts `word`.
    ///
    /// This default implementation runs from [`initial_state`](DeterministicAutomaton::initial_state)
    /// via [`run_from`]. If any step is missing (`None`), the word is rejected;
    /// otherwise acceptance is determined by
    /// [`Automaton::is_accepting_state`](crate::general::Automaton::is_accepting_state)
    /// on the final state.
    fn accepts(&self, word: &[Self::Input]) -> bool {
        let state = self.initial_state();
        let Some(last_state) = self.run_from(state, word) else {
            return false;
        };
        self.is_accepting_state(last_state)
    }
}

