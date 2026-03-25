use std::collections::{HashSet, VecDeque};

use crate::general::automaton::Automaton;
use crate::general::deterministic::DeterministicAutomaton;

pub trait NonDeterministicAutomaton: Automaton {
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a;
    fn successors<'a>(&'a self, state: Self::State, input: &Self::Input) -> impl Iterator<Item = Self::State> + 'a;

    fn reachable_states(&self) -> HashSet<Self::State> {
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();

        for initial_state in self.initial_states() {
            queue.push_back(initial_state);
        }

        while let Some(state) = queue.pop_front() {
            if reachable.contains(&state) {
                continue;
            }

            reachable.insert(state);

            for input in self.alphabet() {
                for successor in self.successors(state, &input) {
                    queue.push_back(successor);
                }
            }
        }

        reachable
    }

    fn common_alphabet(&self, other: &Self) -> HashSet<Self::Input> {
        let alphabet1: HashSet<Self::Input> = self.alphabet().collect();
        let alphabet2: HashSet<Self::Input> = other.alphabet().collect();
        alphabet1.intersection(&alphabet2).cloned().collect()
    }

    fn accepting_states_compatible_with(&self, other: &Self) -> HashSet<Self::State> {
        let mut common = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let common_alphabet = self.common_alphabet(other);

        for initial_state1 in self.initial_states() {
            for initial_state2 in other.initial_states() {
                queue.push_back((initial_state1, initial_state2));
            }
        }

        while let Some((state1, state2)) = queue.pop_front() {
            if visited.contains(&(state1, state2)) {
                continue;
            }

            visited.insert((state1, state2));

            if self.is_accepting_state(state1) && other.is_accepting_state(state2) {
                common.insert(state1);
            }

            for input in &common_alphabet {
                for new_state1 in self.successors(state1, input) {
                    for new_state2 in other.successors(state2, input) {
                        queue.push_back((new_state1, new_state2));
                    }
                }
            }
        }

        common
    }
}

impl<T: DeterministicAutomaton> NonDeterministicAutomaton for T {
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a {
        core::iter::once(self.initial_state())
    }

    fn successors<'a>(&'a self, state: Self::State, input: &Self::Input) -> impl Iterator<Item = Self::State> + 'a {
        self.transition(state, input).into_iter()
    }
}
