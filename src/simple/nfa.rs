use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use crate::arbitrary::Automaton;
use crate::labeled::arbitrary::DeterministicLabeledAutomaton;
use crate::labeled::finite::DeterministicFiniteLabeledAutomaton;
use crate::arbitrary::NonDeterministicAutomaton;
use crate::finite::FiniteAutomaton;
use crate::finite::NonDeterministicFiniteAutomaton;
use crate::labeled::arbitrary::LabeledAutomaton;
use crate::labeled::arbitrary::NonDeterministicLabeledAutomaton;
use crate::labeled::finite::FiniteLabeledAutomaton;
use crate::labeled::finite::NonDeterministicFiniteLabeledAutomaton;

use super::dfa::SimpleDFA;
use super::SimpleBuildError;
use super::state::SimpleNFAState;

/// A small reference implementation of a nondeterministic finite automaton.
///
/// `SimpleNFA` uses dense states `[0..state_count)` and stores transitions
/// as sets:
/// `State × Input -> HashSet<State>`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleNFA {
    initial: HashSet<SimpleNFAState>,
    accepting: HashSet<SimpleNFAState>,
    alphabet: HashSet<char>,
    transitions: Vec<HashMap<char, HashSet<SimpleNFAState>>>,
}

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
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let initial: HashSet<_> = initial.into_iter().collect();
        let accepting: HashSet<_> = accepting.into_iter().collect();
        let mut rows: Vec<HashMap<char, HashSet<SimpleNFAState>>> =
            vec![HashMap::new(); state_count];
        for (q, a, p) in transitions {
            rows[q].entry(a).or_default().insert(p);
        }
        Self {
            initial,
            accepting,
            alphabet,
            transitions: rows,
        }
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
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let initial: HashSet<_> = initial.into_iter().collect();
        let accepting: HashSet<_> = accepting.into_iter().collect();
        for &s in initial.union(&accepting) {
            if s >= state_count {
                return Err(SimpleBuildError::StateOutOfRange {
                    state: s,
                    state_count,
                });
            }
        }
        let mut rows: Vec<HashMap<char, HashSet<SimpleNFAState>>> =
            vec![HashMap::new(); state_count];
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
            rows[q].entry(a).or_default().insert(p);
        }
        Ok(Self {
            initial,
            accepting,
            alphabet,
            transitions: rows,
        })
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
        SimpleNFA {
            initial: self.accepting.clone(),
            accepting: self.initial.clone(),
            alphabet: self.alphabet.clone(),
            transitions: rows,
        }
    }

    fn to_simple_dfa(&self) -> SimpleDFA {
        let mut alphabet_vec: Vec<char> = self.alphabet.iter().copied().collect();
        alphabet_vec.sort_unstable();
        let start: BTreeSet<SimpleNFAState> = self.initial.iter().copied().collect();
        let mut subset_to_id: HashMap<BTreeSet<SimpleNFAState>, usize> = HashMap::new();
        let mut queue: VecDeque<BTreeSet<SimpleNFAState>> = VecDeque::new();
        subset_to_id.insert(start.clone(), 0);
        let mut next_id = 1usize;
        queue.push_back(start);

        let mut trans_out: HashMap<(usize, char), usize> = HashMap::new();

        while let Some(sub) = queue.pop_front() {
            let sid = subset_to_id[&sub];
            for &a in &alphabet_vec {
                let mut dest: BTreeSet<SimpleNFAState> = BTreeSet::new();
                for &s in &sub {
                    if let Some(tos) = self.transitions[s].get(&a) {
                        dest.extend(tos.iter().copied());
                    }
                }
                let tid = if let Some(&id) = subset_to_id.get(&dest) {
                    id
                } else {
                    let id = next_id;
                    next_id += 1;
                    subset_to_id.insert(dest.clone(), id);
                    queue.push_back(dest);
                    id
                };
                trans_out.insert((sid, a), tid);
            }
        }

        let num_states = next_id;
        let accepting_dfa: HashSet<usize> = subset_to_id
            .iter()
            .filter(|(set, _)| set.iter().any(|s| self.accepting.contains(s)))
            .map(|(_, id)| *id)
            .collect();

        let edges: Vec<(usize, char, usize)> =
            trans_out.into_iter().map(|((q, a), p)| (q, a, p)).collect();

        SimpleDFA::new_unchecked(
            num_states,
            0,
            accepting_dfa,
            self.alphabet.iter().copied(),
            edges,
        )
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
        for &s in &self.accepting {
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
}

impl LabeledAutomaton<()> for SimpleNFA {
    type State = SimpleNFAState;
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
        self.initial.contains(&state)
    }
    
    fn get_label(&self, state: Self::State) -> Option<()> {
        if self.accepting.contains(&state) {
            Some(())
        } else {
            None
        }
    }
}

impl Automaton for SimpleNFA {
    fn is_accepting_state(&self, state: Self::State) -> bool {
        self.accepting.contains(&state)
    }
}

impl FiniteLabeledAutomaton<()> for SimpleNFA {
    fn alphabet_set(&self) -> HashSet<Self::Input> {
        self.alphabet.clone()
    }
}

impl FiniteAutomaton for SimpleNFA {
    fn accepting_states_set(&self) -> HashSet<Self::State> {
        self.accepting.clone()
    }
}

impl NonDeterministicLabeledAutomaton<()> for SimpleNFA {
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a
    where
        Self::State: 'a 
    {
        self.initial.iter().copied()
    }
    
    fn successors<'a>(
        &'a self,
        state: Self::State,
        input: &Self::Input,
    ) -> impl Iterator<Item = Self::State> + 'a
    where
        Self::State: 'a 
    {
        self.transitions
            .get(state)
            .and_then(|row| row.get(input))
            .into_iter()
            .flat_map(|s| s.iter().copied())
    }
}

impl NonDeterministicAutomaton for SimpleNFA {}

impl NonDeterministicFiniteLabeledAutomaton<()> for SimpleNFA {
    type CorrespondingDFA = SimpleDFA;

    fn to_dfa_by(&self, _combine: impl Fn((), ()) -> ()) -> Self::CorrespondingDFA {
        self.to_simple_dfa()
    }
    
    fn union(&self, other: &Self) -> Self {
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
        let oi: HashSet<_> = other.initial.iter().map(|&s| s + na).collect();
        let oa: HashSet<_> = other.accepting.iter().map(|&s| s + na).collect();
        SimpleNFA {
            initial: self.initial.union(&oi).copied().collect(),
            accepting: self.accepting.union(&oa).copied().collect(),
            alphabet: ab,
            transitions: rows,
        }
    }
}

impl NonDeterministicFiniteAutomaton for SimpleNFA {
    fn to_dfa(&self) -> SimpleDFA {
        self.to_simple_dfa()
    }

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
        for &fa in &self.accepting {
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
        let other_accepts_empty = other.initial.iter().any(|s| other.accepting.contains(s));
        let mut acc: HashSet<_> = other.accepting.iter().map(|&s| s + na).collect();
        if other_accepts_empty {
            acc.extend(self.accepting.iter().copied());
        }
        SimpleNFA {
            initial: self.initial.clone(),
            accepting: acc,
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
    pub fn star_nfa(&self) -> SimpleNFA {
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

        for &f in &self.accepting {
            let fq = shift(f);
            for &i in &self.initial {
                for (a, tos) in &self.transitions[i] {
                    for &p in tos {
                        rows[fq].entry(*a).or_default().insert(shift(p));
                    }
                }
            }
        }

        let mut accepting: HashSet<_> = self.accepting.iter().map(|&f| shift(f)).collect();
        accepting.insert(0);

        SimpleNFA {
            initial: HashSet::from([0]),
            accepting,
            alphabet: self.alphabet.clone(),
            transitions: rows,
        }
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
            .accepting
            .iter()
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
            .filter(|(k, _)| self.accepting.contains(&k.0) && other.accepting.contains(&k.1))
            .map(|(_, id)| *id)
            .collect();

        let edges: Vec<(usize, char, usize)> = trans
            .into_iter()
            .flat_map(|((q, a), tos)| tos.into_iter().map(move |p| (q, a, p)))
            .collect();

        SimpleNFA::new_unchecked(n, initial, accepting, alphabet.iter().copied(), edges)
    }
}
