use std::collections::{HashMap, HashSet, VecDeque};
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
    pub fn map_labels<NewLabel: Hash + Eq + Clone>(
        &self,
        f: impl Fn(Label) -> NewLabel,
    ) -> SimpleLabeledDFA<NewLabel> {
        SimpleLabeledDFA {
            initial: self.initial,
            labels: self
                .labels
                .iter()
                .map(|(&k, v)| (k, f(v.clone())))
                .collect(),
            alphabet: self.alphabet.clone(),
            transitions: self.transitions.clone(),
        }
    }

    /// Minimize using Hopcroft's algorithm.
    ///
    /// The automaton is completed first. The initial partition groups states by
    /// [`get_label`](LabeledAutomaton::get_label) (`Option<Label>`), not by
    /// accepting vs non-accepting alone.
    pub fn hopcroft_minimize(&self) -> Self {
        let dfa = self.completed();
        let n = dfa.transitions.len();
        let mut alphabet_sorted: Vec<char> = dfa.alphabet.iter().copied().collect();
        alphabet_sorted.sort_unstable();
        let sym_idx: HashMap<char, usize> = alphabet_sorted
            .iter()
            .enumerate()
            .map(|(i, &c)| (c, i))
            .collect();
        let sigma = alphabet_sorted.len();

        let mut inverse: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); sigma]; n];
        for q in 0..n {
            for (&a, &p) in &dfa.transitions[q] {
                if let Some(&i) = sym_idx.get(&a) {
                    inverse[p][i].push(q);
                }
            }
        }

        let mut groups: HashMap<Option<Label>, Vec<usize>> = HashMap::new();
        for q in 0..n {
            let key = dfa.labels.get(&q).cloned();
            groups.entry(key).or_default().push(q);
        }
        let mut blocks: Vec<Vec<usize>> = groups.into_values().collect();
        for b in &mut blocks {
            b.sort_unstable();
        }
        blocks.sort_by_key(|b| b[0]);

        let mut block_of = vec![0usize; n];
        for (bid, states) in blocks.iter().enumerate() {
            for &q in states {
                block_of[q] = bid;
            }
        }

        let mut in_queue = vec![false; blocks.len()];
        let mut waiting = VecDeque::new();

        fn try_enqueue(in_queue: &mut [bool], waiting: &mut VecDeque<usize>, bid: usize) {
            if bid < in_queue.len() && !in_queue[bid] {
                in_queue[bid] = true;
                waiting.push_back(bid);
            }
        }

        for bid in 0..blocks.len() {
            try_enqueue(&mut in_queue, &mut waiting, bid);
        }

        while let Some(b_b) = waiting.pop_front() {
            if b_b >= in_queue.len() || !in_queue[b_b] {
                continue;
            }
            in_queue[b_b] = false;
            if blocks[b_b].is_empty() {
                continue;
            }

            for si in 0..sigma {
                let mut pred_set: HashSet<usize> = HashSet::new();
                for &q in &blocks[b_b] {
                    for &pre in &inverse[q][si] {
                        pred_set.insert(pre);
                    }
                }
                if pred_set.is_empty() {
                    continue;
                }

                let mut s_bid = 0;
                while s_bid < blocks.len() {
                    if blocks[s_bid].is_empty() {
                        s_bid += 1;
                        continue;
                    }
                    let intersects = blocks[s_bid].iter().any(|&q| pred_set.contains(&q));
                    let has_outside = blocks[s_bid].iter().any(|&q| !pred_set.contains(&q));
                    if !intersects || !has_outside {
                        s_bid += 1;
                        continue;
                    }

                    let was_in_w = in_queue[s_bid];
                    in_queue[s_bid] = false;

                    let mut inside = Vec::new();
                    let mut outside = Vec::new();
                    for &q in &blocks[s_bid] {
                        if pred_set.contains(&q) {
                            inside.push(q);
                        } else {
                            outside.push(q);
                        }
                    }
                    inside.sort_unstable();
                    outside.sort_unstable();

                    let new_bid = blocks.len();
                    blocks[s_bid] = outside;
                    blocks.push(inside);
                    in_queue.push(false);

                    for &q in &blocks[new_bid] {
                        block_of[q] = new_bid;
                    }

                    if was_in_w {
                        try_enqueue(&mut in_queue, &mut waiting, s_bid);
                        try_enqueue(&mut in_queue, &mut waiting, new_bid);
                    } else if blocks[s_bid].len() <= blocks[new_bid].len() {
                        try_enqueue(&mut in_queue, &mut waiting, s_bid);
                    } else {
                        try_enqueue(&mut in_queue, &mut waiting, new_bid);
                    }

                    s_bid += 1;
                }
            }
        }

        let k = blocks.len();
        let rep: Vec<usize> = blocks
            .iter()
            .map(|states| *states.iter().min().expect("non-empty block"))
            .collect();

        let mut transitions = vec![HashMap::new(); k];
        let mut labels = HashMap::new();
        for bid in 0..k {
            let q = rep[bid];
            if let Some(l) = dfa.labels.get(&q) {
                labels.insert(bid, l.clone());
            }
            for &a in &alphabet_sorted {
                let p = dfa.transitions[q][&a];
                let nb = block_of[p];
                transitions[bid].insert(a, nb);
            }
        }

        SimpleLabeledDFA {
            initial: block_of[dfa.initial],
            labels,
            alphabet: dfa.alphabet.clone(),
            transitions,
        }
    }

    fn completed(&self) -> Self {
        if self.alphabet.is_empty() {
            return self.clone();
        }
        let n = self.transitions.len();
        let already_total = (0..n).all(|q| {
            self.alphabet
                .iter()
                .all(|&a| self.transitions[q].contains_key(&a))
        });
        if already_total {
            return self.clone();
        }
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

impl<Label: Hash + Eq + Clone> DeterministicFiniteLabeledAutomaton<Label>
    for SimpleLabeledDFA<Label>
{
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
        self.hopcroft_minimize()
    }
}
