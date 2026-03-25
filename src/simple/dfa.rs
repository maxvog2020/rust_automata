use std::collections::{BTreeSet, HashMap, HashSet};

use crate::finite::automaton::FiniteAutomaton;
use crate::finite::deterministic::DeterministicFiniteAutomaton;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::automaton::Automaton;
use crate::general::deterministic::DeterministicAutomaton;

use super::nfa::SimpleNFA;
use super::state::SimpleDFAState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleDFA {
    pub(crate) initial: SimpleDFAState,
    pub(crate) accepting: HashSet<SimpleDFAState>,
    pub(crate) alphabet: BTreeSet<char>,
    pub(crate) transitions: Vec<HashMap<char, SimpleDFAState>>,
}

impl SimpleDFA {
    // TODO: remove asserts
    pub fn new_unchecked(
        state_count: usize,
        initial: SimpleDFAState,
        accepting: impl IntoIterator<Item = SimpleDFAState>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleDFAState, char, SimpleDFAState)>,
    ) -> Self {
        let alphabet: BTreeSet<char> = alphabet.into_iter().collect();
        assert!(initial < state_count, "SimpleDFA: initial out of range");
        let accepting: HashSet<_> = accepting.into_iter().collect();
        let mut rows = vec![HashMap::new(); state_count];
        for (q, a, p) in transitions {
            assert!(q < state_count && p < state_count, "SimpleDFA: transition endpoint out of range");
            assert!(
                alphabet.contains(&a),
                "SimpleDFA: symbol {a:?} not in alphabet"
            );
            assert!(
                rows[q].insert(a, p).is_none(),
                "SimpleDFA: duplicate transition ({q}, {a:?})"
            );
        }
        Self {
            initial,
            accepting,
            alphabet,
            transitions: rows,
        }
    }
}

impl Automaton for SimpleDFA {
    type State = SimpleDFAState;
    type Input = char;

    fn states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a {
        0..self.transitions.len()
    }

    fn alphabet<'a>(&'a self) -> impl Iterator<Item = Self::Input> + 'a {
        self.alphabet.iter().copied()
    }

    fn is_valid_state(&self, state: Self::State) -> bool {
        state < self.transitions.len()
    }

    fn is_initial_state(&self, state: Self::State) -> bool {
        state == self.initial
    }

    fn is_accepting_state(&self, state: Self::State) -> bool {
        self.accepting.contains(&state)
    }
}

impl FiniteAutomaton for SimpleDFA {}

impl DeterministicAutomaton for SimpleDFA {
    fn initial_state(&self) -> Self::State {
        self.initial
    }

    fn transition(&self, state: Self::State, input: &Self::Input) -> Option<Self::State> {
        self.transitions.get(state)?.get(input).copied()
    }
}

impl DeterministicFiniteAutomaton for SimpleDFA {
    fn to_nfa(&self) -> impl NonDeterministicFiniteAutomaton {
        let transitions = self.transitions.iter().map(|transition| {
            transition.iter().map(|(&a, &p)| (a, HashSet::from([p]))).collect()
        }).collect();

        SimpleNFA {
            initial: HashSet::from([self.initial]),
            accepting: self.accepting.clone(),
            alphabet: self.alphabet.clone(),
            transitions,
        }
    }

    fn complete(&self) -> impl DeterministicFiniteAutomaton {
        let transitions = self.states().map(|s| {
            self.alphabet().map(|a| (a, s)).collect()
        }).collect();

        SimpleDFA {
            initial: self.initial,
            accepting: self.accepting.clone(),
            alphabet: self.alphabet.clone(),
            transitions,
        }
    }
}
