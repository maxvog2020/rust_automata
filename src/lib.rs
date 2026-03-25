use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

pub trait Automaton {
    type State: Hash + Eq + Copy;
    type Input: Hash + Eq + Clone;
    
    fn states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a;
    fn alphabet<'a>(&'a self) -> impl Iterator<Item = Self::Input> + 'a;

    fn is_valid_state(&self, state: Self::State) -> bool;
    fn is_initial_state(&self, state: Self::State) -> bool;
    fn is_accepting_state(&self, state: Self::State) -> bool;
}

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
        return self.is_accepting_state(state);
    }
}

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
        let alphabet1 = self.alphabet().collect::<HashSet<Self::Input>>();
        let alphabet2 = other.alphabet().collect::<HashSet<Self::Input>>();
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
    
            for input in common_alphabet.iter() {
                for new_state1 in self.successors(state1, &input) {
                    for new_state2 in other.successors(state2, &input) {
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
        std::iter::once(self.initial_state())
    }

    fn successors<'a>(&'a self, state: Self::State, input: &Self::Input) -> impl Iterator<Item = Self::State> + 'a {
        self.transition(state, input).into_iter()
    }
}

pub trait NonDeterministicFiniteAutomaton: NonDeterministicAutomaton {
    fn to_dfa(&self) -> impl DeterministicFiniteAutomaton;

    fn union(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    fn concatenate(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;

    fn star(&self) -> Self;
    fn reverse(&self) -> Self;
    fn complement(&self) -> Self;
}

// No complex operations on purpose, convert to NFA first
pub trait DeterministicFiniteAutomaton: DeterministicAutomaton {
    fn to_nfa(&self) -> impl NonDeterministicFiniteAutomaton;

    fn minimize(&self) -> impl DeterministicFiniteAutomaton; // TODO: default implementation based on Brzozowski's algorithm
}

// TODO:
pub struct SimpleDFA;
pub struct SimpleNFA;

// impl DeterministicFiniteAutomaton for SimpleDFA
// impl NonDeterministicFiniteAutomaton for SimpleNFA

