use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::labeled::arbitrary::DeterministicLabeledAutomaton;
use crate::labeled::arbitrary::LabeledAutomaton;
use crate::labeled::finite::DeterministicFiniteLabeledAutomaton;
use crate::labeled::finite::FiniteLabeledAutomaton;

use super::error::SimpleBuildError;
use super::nfa::SimpleLabeledNFA;
use super::state::SimpleLabeledDFAState;

/// A small reference implementation of a deterministic finite automaton.
///
/// `SimpleDFA` uses a dense state set `[0..state_count)` with transitions
/// stored as `State × Input -> Option<State>`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleLabeledDFA<Label: Hash + Eq + Clone> {
    pub(crate) initial: SimpleLabeledDFAState,
    pub(crate) labels: HashMap<SimpleLabeledDFAState, Label>,
    pub(crate) alphabet: HashSet<char>,
    pub(crate) transitions: Vec<HashMap<char, SimpleLabeledDFAState>>,
}

impl<Label: Hash + Eq + Clone> SimpleLabeledDFA<Label> {
    /// Construct a `SimpleDFA` without validating invariants.
    ///
    /// This constructor is intended for internal use and tests. It assumes:
    /// - `initial < state_count`
    /// - accepting states are within `0..state_count`
    /// - all transition endpoints are within range
    /// - transition symbols belong to `alphabet`
    /// - at most one transition per `(state, symbol)`
    pub fn new_labeled_unchecked(
        state_count: usize,
        initial: SimpleLabeledDFAState,
        labels: impl IntoIterator<Item = (SimpleLabeledDFAState, Label)>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleLabeledDFAState, char, SimpleLabeledDFAState)>,
    ) -> Self {
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let labels: HashMap<_, _> = labels.into_iter().collect();
        let mut rows = vec![HashMap::new(); state_count];
        for (q, a, p) in transitions {
            rows[q].insert(a, p);
        }
        Self {
            initial,
            labels,
            alphabet,
            transitions: rows,
        }
    }

    /// Construct a `SimpleDFA` with validation.
    ///
    /// See [`SimpleBuildError`] for possible failures.
    pub fn try_new_labeled(
        state_count: usize,
        initial: SimpleLabeledDFAState,
        labels: impl IntoIterator<Item = (SimpleLabeledDFAState, Label)>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleLabeledDFAState, char, SimpleLabeledDFAState)>,
    ) -> Result<Self, SimpleBuildError> {
        if initial >= state_count {
            return Err(SimpleBuildError::InitialOutOfRange {
                initial,
                state_count,
            });
        }
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let labels: HashMap<_, _> = labels.into_iter().collect();
        for (&s, _) in &labels {
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
                return Err(SimpleBuildError::TransitionToOutOfRange { to: p, state_count });
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
            labels,
            alphabet,
            transitions: rows,
        })
    }

    /// Convert this DFA into a dense transition matrix `M[state][input]`.
    ///
    /// Each cell contains `Some(next_state)` if the transition exists for the
    /// symbol, otherwise `None`.
    pub fn to_matrix(&self) -> Vec<Vec<Option<SimpleLabeledDFAState>>> {
        self.states()
            .map(|s| {
                self.alphabet()
                    .map(|a| self.transition(s, &a))
                    .collect::<Vec<Option<SimpleLabeledDFAState>>>()
            })
            .collect()
    }

    // TODO: docs
    pub fn map_labels<NewLabel: Hash + Eq + Clone>(&self, f: impl Fn(Label) -> NewLabel) -> SimpleLabeledDFA<NewLabel> {
        SimpleLabeledDFA {
            initial: self.initial,
            labels: self.labels.iter().map(|(&k, v)| (k, f(v.clone()))).collect(),
            alphabet: self.alphabet.clone(),
            transitions: self.transitions.clone(),
        }
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
            labels: self.labels.clone(),
            alphabet: self.alphabet.clone(),
            transitions: rows,
        }
    }
}

impl<Label: Hash + Eq + Clone> LabeledAutomaton<Label> for SimpleLabeledDFA<Label> {
    type State = SimpleLabeledDFAState;
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
    
    fn get_label(&self, state: Self::State) -> Option<Label> {
        self.labels.get(&state).cloned()
    }
}

impl<Label: Hash + Eq + Clone> FiniteLabeledAutomaton<Label> for SimpleLabeledDFA<Label> {
    fn alphabet_set(&self) -> HashSet<Self::Input> {
        self.alphabet.clone()
    }
}

impl<Label: Hash + Eq + Clone> DeterministicLabeledAutomaton<Label> for SimpleLabeledDFA<Label> {
    fn initial_state(&self) -> Self::State {
        self.initial
    }

    fn transition(&self, state: Self::State, input: &Self::Input) -> Option<Self::State> {
        self.transitions.get(state)?.get(input).copied()
    }
}

impl<Label: Hash + Eq + Clone> DeterministicFiniteLabeledAutomaton<Label> for SimpleLabeledDFA<Label> {
    type CorrespondingNFA = SimpleLabeledNFA<Label>;
    
    fn to_nfa(&self) -> Self::CorrespondingNFA {
        let edges: Vec<(usize, char, usize)> = self
            .transitions
            .iter()
            .enumerate()
            .flat_map(|(q, transition)| transition.iter().map(move |(&a, &p)| (q, a, p)))
            .collect();

        SimpleLabeledNFA::new_labeled_unchecked(
            self.transitions.len(),
            [self.initial],
            self.labels.iter().map(|(s, l)| (*s, l.clone())),
            self.alphabet.iter().copied(),
            edges,
        )
    }
    
    fn complete(&self) -> Self {
        self.completed()
    }
    
    fn minimize(&self) -> Self {
        todo!()
    }
}

