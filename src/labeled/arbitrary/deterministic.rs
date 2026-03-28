use std::hash::Hash;

use crate::labeled::arbitrary::automaton::LabeledAutomaton;

// TODO: docs
pub trait DeterministicLabeledAutomaton<Label: Hash + Eq + Clone>: LabeledAutomaton<Label> {
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

    // TODO: docs
    fn get_label_of_word(&self, word: &[Self::Input]) -> Option<Label> {
        let state = self.initial_state();
        let Some(last_state) = self.run_from(state, word) else {
            return None;
        };
        self.get_label(last_state)
    }
}
