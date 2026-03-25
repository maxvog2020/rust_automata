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

    /// Check whether the automaton accepts `word`.
    ///
    /// This default implementation repeatedly applies [`transition`] to
    /// consume the input. If at any step the transition is missing
    /// (`None`), the word is rejected.
    fn accepts(&self, word: &[Self::Input]) -> bool {
        let mut state = self.initial_state();
        for input in word {
            let Some(new_state) = self.transition(state, input) else {
                return false;
            };
            state = new_state;
        }
        self.is_accepting_state(state)
    }
}
