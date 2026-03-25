use crate::general::automaton::Automaton;

pub trait DeterministicAutomaton: Automaton {
    fn initial_state(&self) -> Self::State;
    fn transition(&self, state: Self::State, input: &Self::Input) -> Option<Self::State>;

    fn accepts(&self, _word: &[Self::Input]) -> bool {
        todo!("DeterministicAutomaton::accepts")
    }
}
