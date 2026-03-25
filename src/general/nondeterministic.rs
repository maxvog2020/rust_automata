use std::collections::HashSet;

use crate::general::automaton::Automaton;
use crate::general::deterministic::DeterministicAutomaton;

pub trait NonDeterministicAutomaton: Automaton {
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a;

    fn successors<'a>(
        &'a self,
        state: Self::State,
        input: &Self::Input,
    ) -> impl Iterator<Item = Self::State> + 'a;

    fn accepts(&self, _word: &[Self::Input]) -> bool {
        todo!("NonDeterministicAutomaton::accepts")
    }

    fn reachable_states(&self) -> HashSet<Self::State> {
        todo!("NonDeterministicAutomaton::reachable_states")
    }

    fn common_alphabet(&self, _other: &Self) -> HashSet<Self::Input> {
        todo!("NonDeterministicAutomaton::common_alphabet")
    }

    fn accepting_states_compatible_with(&self, _other: &Self) -> HashSet<Self::State> {
        todo!("NonDeterministicAutomaton::accepting_states_compatible_with")
    }
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
