use std::collections::{HashMap, HashSet};

use crate::finite::automaton::FiniteAutomaton;
use crate::finite::deterministic::DeterministicFiniteAutomaton;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::automaton::Automaton;
use crate::general::deterministic::DeterministicAutomaton;

use super::error::SimpleBuildError;
use super::nfa::SimpleNFA;
use super::state::SimpleDFAState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleDFA {
    pub(crate) initial: SimpleDFAState,
    pub(crate) accepting: HashSet<SimpleDFAState>,
    pub(crate) alphabet: HashSet<char>,
    pub(crate) transitions: Vec<HashMap<char, SimpleDFAState>>,
}

impl SimpleDFA {
    pub fn new_unchecked(
        state_count: usize,
        initial: SimpleDFAState,
        accepting: impl IntoIterator<Item = SimpleDFAState>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleDFAState, char, SimpleDFAState)>,
    ) -> Self {
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let accepting: HashSet<_> = accepting.into_iter().collect();
        let mut rows = vec![HashMap::new(); state_count];
        for (q, a, p) in transitions {
            rows[q].insert(a, p);
        }
        Self {
            initial,
            accepting,
            alphabet,
            transitions: rows,
        }
    }

    pub fn try_new(
        state_count: usize,
        initial: SimpleDFAState,
        accepting: impl IntoIterator<Item = SimpleDFAState>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleDFAState, char, SimpleDFAState)>,
    ) -> Result<Self, SimpleBuildError> {
        if initial >= state_count {
            return Err(SimpleBuildError::InitialOutOfRange {
                initial,
                state_count,
            });
        }
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let accepting: HashSet<_> = accepting.into_iter().collect();
        for &s in &accepting {
            if s >= state_count {
                return Err(SimpleBuildError::StateOutOfRange {
                    state: s,
                    state_count,
                });
            }
        }
        let mut rows = vec![HashMap::new(); state_count];
        for (q, a, p) in transitions {
            if q >= state_count {
                return Err(SimpleBuildError::TransitionFromOutOfRange {
                    from: q,
                    state_count,
                });
            }
            if p >= state_count {
                return Err(SimpleBuildError::TransitionToOutOfRange {
                    to: p,
                    state_count,
                });
            }
            if !alphabet.contains(&a) {
                return Err(SimpleBuildError::SymbolNotInAlphabet(a));
            }
            if rows[q].insert(a, p).is_some() {
                return Err(SimpleBuildError::DuplicateDeterministicTransition {
                    state: q,
                    symbol: a,
                });
            }
        }
        Ok(Self {
            initial,
            accepting,
            alphabet,
            transitions: rows,
        })
    }

    pub(crate) fn completed(&self) -> Self {
        if self.alphabet.is_empty() {
            return self.clone();
        }
        let n = self.transitions.len();
        let sink = n;
        let mut rows = self.transitions.clone();
        rows.push(HashMap::new());
        for row in rows.iter_mut().take(n) {
            for &a in &self.alphabet {
                row.entry(a).or_insert(sink);
            }
        }
        for &a in &self.alphabet {
            rows[sink].insert(a, sink);
        }
        Self {
            initial: self.initial,
            accepting: self.accepting.clone(),
            alphabet: self.alphabet.clone(),
            transitions: rows,
        }
    }

    pub(crate) fn complement_total(&self) -> Self {
        let n = self.transitions.len();
        let accepting = (0..n).filter(|q| !self.accepting.contains(q)).collect();
        Self {
            initial: self.initial,
            accepting,
            alphabet: self.alphabet.clone(),
            transitions: self.transitions.clone(),
        }
    }

    pub(crate) fn as_simple_nfa(&self) -> SimpleNFA {
        let transitions = self.transitions.iter().map(|transition| {
            transition
                .iter()
                .map(|(&a, &p)| (a, HashSet::from([p])))
                .collect()
        }).collect();

        SimpleNFA {
            initial: HashSet::from([self.initial]),
            accepting: self.accepting.clone(),
            alphabet: self.alphabet.clone(),
            transitions,
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
        self.as_simple_nfa()
    }

    fn complete(&self) -> impl DeterministicFiniteAutomaton {
        self.completed()
    }

    fn minimize(&self) -> impl DeterministicFiniteAutomaton {
        self.as_simple_nfa()
            .reversed()
            .to_simple_dfa()
            .as_simple_nfa()
            .reversed()
            .to_simple_dfa()
    }
}
