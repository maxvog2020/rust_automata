use std::collections::{HashMap, HashSet};

use crate::finite::FiniteAutomaton;
use crate::finite::DeterministicFiniteAutomaton;
use crate::arbitrary::Automaton;
use crate::arbitrary::DeterministicAutomaton;

use super::error::SimpleBuildError;
use super::nfa::SimpleNFA;
use super::state::SimpleDFAState;

/// A small reference implementation of a deterministic finite automaton.
///
/// `SimpleDFA` uses a dense state set `[0..state_count)` with transitions
/// stored as `State × Input -> Option<State>`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleDFA {
    initial: SimpleDFAState,
    accepting: HashSet<SimpleDFAState>,
    alphabet: HashSet<char>,
    transitions: Vec<HashMap<char, SimpleDFAState>>,
}

impl SimpleDFA {
    /// Construct a `SimpleDFA` without validating invariants.
    ///
    /// This constructor is intended for internal use and tests. It assumes:
    /// - `initial < state_count`
    /// - accepting states are within `0..state_count`
    /// - all transition endpoints are within range
    /// - transition symbols belong to `alphabet`
    /// - at most one transition per `(state, symbol)`
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

    /// Construct a `SimpleDFA` with validation.
    ///
    /// See [`SimpleBuildError`] for possible failures.
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

    /// Convert this DFA into a dense transition matrix `M[state][input]`.
    ///
    /// Each cell contains `Some(next_state)` if the transition exists for the
    /// symbol, otherwise `None`.
    pub fn to_matrix(&self) -> Vec<Vec<Option<SimpleDFAState>>> {
        self.states()
            .map(|s| {
                self.alphabet()
                    .map(|a| self.transition(s, &a))
                    .collect::<Vec<Option<SimpleDFAState>>>()
            })
            .collect()
    }

    fn completed(&self) -> Self {
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

impl FiniteAutomaton for SimpleDFA {
    fn alphabet_set(&self) -> HashSet<Self::Input> {
        self.alphabet.clone()
    }

    fn accepting_states_set(&self) -> HashSet<Self::State> {
        self.accepting.clone()
    }
}

impl DeterministicAutomaton for SimpleDFA {
    fn initial_state(&self) -> Self::State {
        self.initial
    }

    fn transition(&self, state: Self::State, input: &Self::Input) -> Option<Self::State> {
        self.transitions.get(state)?.get(input).copied()
    }
}

impl DeterministicFiniteAutomaton for SimpleDFA {
    type CorrespondingNFA = SimpleNFA;

    fn to_nfa(&self) -> SimpleNFA {
        let edges: Vec<(usize, char, usize)> = self
            .transitions
            .iter().enumerate()
            .flat_map(|(q, transition)| {
                transition
                    .iter()
                    .map(move |(&a, &p)| (q, a, p))
            })
            .collect();

        SimpleNFA::new_unchecked(
            self.transitions.len(),
            [self.initial],
            self.accepting.iter().copied(),
            self.alphabet.iter().copied(),
            edges,
        )
    }

    fn complete(&self) -> Self {
        self.completed()
    }
}
