use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

use crate::arbitrary::Automaton;
use crate::labeled::arbitrary::DeterministicLabeledAutomaton;
use crate::labeled::finite::DeterministicFiniteLabeledAutomaton;
use crate::arbitrary::NonDeterministicAutomaton;
use crate::finite::FiniteAutomaton;
use crate::finite::NonDeterministicFiniteAutomaton;
use crate::labeled::arbitrary::LabeledAutomaton;
use crate::labeled::finite::NonDeterministicFiniteLabeledAutomaton;
use crate::labeled::simple::SimpleLabeledNFA;
use crate::utility::{hashmap_of_unit_to_hashset, hashset_of_unit_to_hashmap};

use super::SimpleBuildError;
use super::state::SimpleNFAState;

/// A small reference implementation of a nondeterministic finite automaton.
///
/// `SimpleNFA` uses dense states `[0..state_count)` and stores transitions
/// as sets:
/// `State × Input -> HashSet<State>`.
pub type SimpleNFA = SimpleLabeledNFA<()>;

impl SimpleNFA {
    /// Construct a `SimpleNFA` without validating invariants.
    ///
    /// This constructor is intended for internal use and tests.
    pub fn new_unchecked(
        state_count: usize,
        initial: impl IntoIterator<Item = SimpleNFAState>,
        accepting: impl IntoIterator<Item = SimpleNFAState>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleNFAState, char, SimpleNFAState)>,
    ) -> Self {
        let labels = accepting.into_iter().map(|s| (s, ()));
        SimpleLabeledNFA::new_labeled_unchecked(state_count, initial, labels, alphabet, transitions)
    }

    /// Construct a `SimpleNFA` with validation.
    ///
    /// See [`SimpleBuildError`] for possible failures.
    pub fn try_new(
        state_count: usize,
        initial: impl IntoIterator<Item = SimpleNFAState>,
        accepting: impl IntoIterator<Item = SimpleNFAState>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleNFAState, char, SimpleNFAState)>,
    ) -> Result<Self, SimpleBuildError> {
        let labels = accepting.into_iter().map(|s| (s, ()));
        SimpleLabeledNFA::try_new_labeled(state_count, initial, labels, alphabet, transitions)
    }

    /// Build a **two-state** `SimpleNFA`: initial state `0`, unique accepting
    /// state `1`, and a one-step move from `0` to `1` on selected inputs only.
    ///
    /// Accepted words are exactly the **singleton** (one-letter) words whose
    /// symbol appears in `symbols` (state `0` is not accepting, so the empty
    /// word is rejected).
    ///
    /// Returns [`SimpleBuildError::SymbolNotInAlphabet`] if any `symbols`
    /// entry is missing from `alphabet`.
    pub fn try_new_singleton_words(
        alphabet: impl IntoIterator<Item = char>,
        symbols: impl IntoIterator<Item = char>,
    ) -> Result<Self, SimpleBuildError> {
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let symbols: HashSet<char> = symbols.into_iter().collect();
        for &c in &symbols {
            if !alphabet.contains(&c) {
                return Err(SimpleBuildError::SymbolNotInAlphabet(c));
            }
        }
        let transitions: Vec<_> = symbols.into_iter().map(|c| (0usize, c, 1usize)).collect();
        Self::try_new(2, [0], [1], alphabet, transitions)
    }

    // TODO: docs
    pub fn label_all_accepting_states_with<Label: Hash + Eq + Clone>(&self, label: Label) -> SimpleLabeledNFA<Label> {
        self.map_labels(|_| label.clone())
    }
}

impl Automaton for SimpleNFA {
    fn is_accepting_state(&self, state: Self::State) -> bool {
        self.labels.contains_key(&state)
    }
}

impl FiniteAutomaton for SimpleNFA {
    fn accepting_states_set(&self) -> HashSet<Self::State> {
        self.labels.keys().copied().collect()
    }
}

impl NonDeterministicAutomaton for SimpleNFA {}

impl NonDeterministicFiniteAutomaton for SimpleNFA {
    fn difference(&self, other: &Self) -> Self {
        self.difference_inner(other)
    }

    fn concatenate(&self, other: &Self) -> Self {
        let na = self.transitions.len();
        let nb = other.transitions.len();
        let ab: HashSet<char> = self.alphabet.union(&other.alphabet).copied().collect();
        let mut rows = self.transitions.clone();
        rows.resize(na + nb, HashMap::new());
        for q in 0..nb {
            for (a, tos) in &other.transitions[q] {
                let nt: HashSet<_> = tos.iter().map(|&p| p + na).collect();
                rows[na + q].entry(*a).or_default().extend(nt);
            }
        }
        for &fa in self.labels.keys() {
            for &qb in &other.initial {
                if qb >= other.transitions.len() {
                    continue;
                }
                for (a, tos) in &other.transitions[qb] {
                    let nt: HashSet<_> = tos.iter().map(|&p| p + na).collect();
                    rows[fa].entry(*a).or_default().extend(nt);
                }
            }
        }
        // if `other` accepts the empty word
        let other_accepts_empty = other.initial.iter().any(|s| other.labels.contains_key(s));
        let mut acc: HashSet<_> = other.labels.keys().map(|&s| s + na).collect();
        if other_accepts_empty {
            acc.extend(hashmap_of_unit_to_hashset(self.labels.clone()));
        }
        SimpleNFA {
            initial: self.initial.clone(),
            labels: hashset_of_unit_to_hashmap(acc),
            alphabet: ab,
            transitions: rows,
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        self.intersection_inner(other)
    }

    fn star(&self) -> Self {
        self.star_nfa()
    }

    fn reverse(&self) -> Self {
        self.reversed()
    }

    fn trimmed(&self) -> Self {
        self.restrict_states(
            &self
                .reachable_states_set()
                .intersection(&self.co_reachable())
                .copied()
                .collect(),
        )
    }

    fn complement(&self) -> Self {
        self.complement_inner()
    }

    fn accessible(&self) -> Self {
        self.restrict_states(&self.reachable_states_set())
    }

    fn co_accessible(&self) -> Self {
        self.restrict_states(&self.co_reachable())
    }
}

impl SimpleNFA {
    fn star_nfa(&self) -> SimpleNFA {
        let n = self.transitions.len();
        let new_n = n + 1;
        let shift = |q: usize| q + 1;
        let mut rows: Vec<HashMap<char, HashSet<SimpleNFAState>>> = vec![HashMap::new(); new_n];

        for q in 0..n {
            for (a, tos) in &self.transitions[q] {
                for &p in tos {
                    rows[shift(q)].entry(*a).or_default().insert(shift(p));
                }
            }
        }

        for &i in &self.initial {
            for (a, tos) in &self.transitions[i] {
                for &p in tos {
                    rows[0].entry(*a).or_default().insert(shift(p));
                }
            }
        }

        for &f in self.labels.keys() {
            let fq = shift(f);
            for &i in &self.initial {
                for (a, tos) in &self.transitions[i] {
                    for &p in tos {
                        rows[fq].entry(*a).or_default().insert(shift(p));
                    }
                }
            }
        }

        let mut accepting: HashSet<_> = self.labels.keys().map(|&f| shift(f)).collect();
        accepting.insert(0);

        SimpleNFA {
            initial: HashSet::from([0]),
            labels: hashset_of_unit_to_hashmap(accepting),
            alphabet: self.alphabet.clone(),
            transitions: rows,
        }
    }

    fn reversed(&self) -> SimpleNFA {
        let n = self.transitions.len();
        let mut rows: Vec<HashMap<char, HashSet<SimpleNFAState>>> = vec![HashMap::new(); n];
        for q in 0..n {
            for (a, tos) in &self.transitions[q] {
                for &p in tos {
                    rows[p].entry(*a).or_default().insert(q);
                }
            }
        }

        let new_initial = self.labels.keys().copied().collect();
        let new_labels = self.initial.iter().map(|&s| (s, ())).collect();

        SimpleNFA {
            initial: new_initial,
            labels: new_labels,
            alphabet: self.alphabet.clone(),
            transitions: rows,
        }
    }

    fn co_reachable(&self) -> HashSet<SimpleNFAState> {
        let mut rev: HashMap<SimpleNFAState, HashSet<SimpleNFAState>> = HashMap::new();
        for q in 0..self.transitions.len() {
            for tos in self.transitions[q].values() {
                for &p in tos {
                    rev.entry(p).or_default().insert(q);
                }
            }
        }
        let mut seen = HashSet::new();
        let mut q = VecDeque::new();
        for &s in self.labels.keys() {
            q.push_back(s);
        }
        while let Some(s) = q.pop_front() {
            if !seen.insert(s) {
                continue;
            }
            if let Some(preds) = rev.get(&s) {
                for &p in preds {
                    q.push_back(p);
                }
            }
        }
        seen
    }

    fn complement_inner(&self) -> SimpleNFA {
        let d = self.to_dfa().complete();
        let accepting: HashSet<SimpleNFAState> =
            d.states().filter(|&q| !d.is_accepting_state(q)).collect();
        let mut edges = Vec::new();
        for q in d.states() {
            for a in d.alphabet() {
                if let Some(p) = d.transition(q, &a) {
                    edges.push((q, a, p));
                }
            }
        }
        SimpleNFA::new_unchecked(
            d.states().count(),
            [d.initial_state()],
            accepting,
            d.alphabet(),
            edges,
        )
    }

    fn difference_inner(&self, other: &Self) -> SimpleNFA {
        self.intersection_inner(&other.complement_inner())
    }

    fn restrict_states(&self, keep: &HashSet<SimpleNFAState>) -> SimpleNFA {
        if keep.is_empty() {
            return SimpleNFA::new_unchecked(0, [], [], self.alphabet.iter().copied(), []);
        }
        let mut sorted: Vec<_> = keep.iter().copied().collect();
        sorted.sort_unstable();
        let remap: HashMap<SimpleNFAState, SimpleNFAState> =
            sorted.iter().enumerate().map(|(i, &s)| (s, i)).collect();
        let n = sorted.len();
        let mut edges = Vec::new();
        for &q in &sorted {
            for (a, tos) in &self.transitions[q] {
                for &t in tos {
                    if keep.contains(&t) {
                        edges.push((remap[&q], *a, remap[&t]));
                    }
                }
            }
        }
        let initial: HashSet<_> = self
            .initial
            .iter()
            .filter_map(|&s| remap.get(&s).copied())
            .collect();
        let accepting: HashSet<_> = self
            .labels
            .keys()
            .filter_map(|&s| remap.get(&s).copied())
            .collect();
        let ab = self.alphabet.clone();
        SimpleNFA::new_unchecked(n, initial, accepting, ab.iter().copied(), edges)
    }

    fn intersection_inner(&self, other: &Self) -> SimpleNFA {
        let alphabet: HashSet<char> = self
            .alphabet
            .intersection(&other.alphabet)
            .copied()
            .collect();
        let mut alphabet_vec: Vec<char> = alphabet.iter().copied().collect();
        alphabet_vec.sort_unstable();
        let mut pair_id: HashMap<(SimpleNFAState, SimpleNFAState), usize> = HashMap::new();
        let mut queue: VecDeque<(SimpleNFAState, SimpleNFAState)> = VecDeque::new();
        let mut next_id = 0usize;
        let mut trans: HashMap<(usize, char), HashSet<usize>> = HashMap::new();

        for &i1 in &self.initial {
            for &i2 in &other.initial {
                let p = (i1, i2);
                if pair_id.insert(p, next_id).is_none() {
                    queue.push_back(p);
                    next_id += 1;
                }
            }
        }

        while let Some((s1, s2)) = queue.pop_front() {
            let sid = pair_id[&(s1, s2)];
            for &a in &alphabet_vec {
                let t1s: Vec<_> = self.transitions[s1]
                    .get(&a)
                    .map(|x| x.iter().copied().collect())
                    .unwrap_or_default();
                let t2s: Vec<_> = other.transitions[s2]
                    .get(&a)
                    .map(|x| x.iter().copied().collect())
                    .unwrap_or_default();
                for &p1 in &t1s {
                    for &p2 in &t2s {
                        let pair = (p1, p2);
                        let tid = if let Some(&id) = pair_id.get(&pair) {
                            id
                        } else {
                            let id = next_id;
                            next_id += 1;
                            pair_id.insert(pair, id);
                            queue.push_back(pair);
                            id
                        };
                        trans.entry((sid, a)).or_default().insert(tid);
                    }
                }
            }
        }

        let n = next_id;
        let mut initial = HashSet::new();
        for &i1 in &self.initial {
            for &i2 in &other.initial {
                initial.insert(pair_id[&(i1, i2)]);
            }
        }

        let accepting: HashSet<_> = pair_id
            .iter()
            .filter(|(k, _)| self.labels.contains_key(&k.0) && other.labels.contains_key(&k.1))
            .map(|(_, id)| *id)
            .collect();

        let edges: Vec<(usize, char, usize)> = trans
            .into_iter()
            .flat_map(|((q, a), tos)| tos.into_iter().map(move |p| (q, a, p)))
            .collect();

        SimpleNFA::new_unchecked(n, initial, accepting, alphabet.iter().copied(), edges)
    }
}
