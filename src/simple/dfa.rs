use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::arbitrary::Automaton;
use crate::arbitrary::DeterministicAutomaton;
use crate::finite::DeterministicFiniteAutomaton;
use crate::finite::FiniteAutomaton;
use crate::labeled::arbitrary::DeterministicLabeledAutomaton;
use crate::labeled::arbitrary::LabeledAutomaton;
use crate::labeled::finite::DeterministicFiniteLabeledAutomaton;
use crate::labeled::finite::FiniteLabeledAutomaton;
use crate::labeled::simple::SimpleLabeledDFA;
use crate::utility::flat_vec_hashmap;

use super::SimpleBuildError;
use super::nfa::SimpleNFA;
use super::state::SimpleDFAState;

/// A small reference implementation of a deterministic finite automaton.
///
/// `SimpleDFA` uses a dense state set `[0..state_count)` with transitions
/// stored as `State × Input -> Option<State>`.
pub type SimpleDFA = SimpleLabeledDFA<()>;

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
        let labels = accepting.into_iter().map(|s| (s, ()));
        SimpleLabeledDFA::new_labeled_unchecked(state_count, initial, labels, alphabet, transitions)
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
        let labels = accepting.into_iter().map(|s| (s, ()));
        SimpleLabeledDFA::try_new_labeled(state_count, initial, labels, alphabet, transitions)
    }

    // TODO: docs
    pub fn label_all_accepting_states_with<Label: Hash + Eq + Clone>(&self, label: Label) -> SimpleLabeledDFA<Label> {
        self.map_labels(|_| label.clone())
    }
}

impl Automaton for SimpleDFA {
    fn is_accepting_state(&self, state: Self::State) -> bool {
        self.labels.contains_key(&state)
    }
}

impl FiniteAutomaton for SimpleDFA {
    fn accepting_states_set(&self) -> HashSet<Self::State> {
        self.labels.keys().copied().collect()
    }
}

impl DeterministicAutomaton for SimpleDFA {}

impl DeterministicFiniteAutomaton for SimpleDFA {}
