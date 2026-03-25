use crate::general::automaton::Automaton;

pub trait DeterministicAutomaton: Automaton {
    fn initial_state(&self) -> Self::State;
    fn transition(&self, state: Self::State, input: &Self::Input) -> Option<Self::State>;

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
