use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::hash::Hash;

use crate::labeled::arbitrary::LabeledAutomaton;
use crate::labeled::arbitrary::NonDeterministicLabeledAutomaton;
use crate::labeled::finite::FiniteLabeledAutomaton;
use crate::labeled::finite::NonDeterministicFiniteLabeledAutomaton;

use super::dfa::SimpleLabeledDFA;
use super::error::SimpleBuildError;
use super::state::SimpleLabeledNFAState;

/// A small reference implementation of a nondeterministic finite automaton.
///
/// `SimpleNFA` uses dense states `[0..state_count)` and stores transitions
/// as sets:
/// `State × Input -> HashSet<State>`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleLabeledNFA<Label: Hash + Eq + Clone> {
    pub(crate) initial: HashSet<SimpleLabeledNFAState>,
    pub(crate) labels: HashMap<SimpleLabeledNFAState, Label>,
    pub(crate) alphabet: HashSet<char>,
    pub(crate) transitions: Vec<HashMap<char, HashSet<SimpleLabeledNFAState>>>,
}

impl<Label: Hash + Eq + Clone> SimpleLabeledNFA<Label> {
    /// Construct a `SimpleNFA` without validating invariants.
    ///
    /// This constructor is intended for internal use and tests.
    pub fn new_labeled_unchecked(
        state_count: usize,
        initial: impl IntoIterator<Item = SimpleLabeledNFAState>,
        labels: impl IntoIterator<Item = (SimpleLabeledNFAState, Label)>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleLabeledNFAState, char, SimpleLabeledNFAState)>,
    ) -> Self {
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let initial: HashSet<_> = initial.into_iter().collect();
        let labels: HashMap<_, _> = labels.into_iter().collect();
        let mut rows: Vec<HashMap<char, HashSet<SimpleLabeledNFAState>>> =
            vec![HashMap::new(); state_count];
        for (q, a, p) in transitions {
            rows[q].entry(a).or_default().insert(p);
        }
        Self {
            initial,
            labels,
            alphabet,
            transitions: rows,
        }
    }

    /// Construct a `SimpleNFA` with validation.
    ///
    /// See [`SimpleBuildError`] for possible failures.
    pub fn try_new_labeled(
        state_count: usize,
        initial: impl IntoIterator<Item = SimpleLabeledNFAState>,
        labels: impl IntoIterator<Item = (SimpleLabeledNFAState, Label)>,
        alphabet: impl IntoIterator<Item = char>,
        transitions: impl IntoIterator<Item = (SimpleLabeledNFAState, char, SimpleLabeledNFAState)>,
    ) -> Result<Self, SimpleBuildError> {
        let alphabet: HashSet<char> = alphabet.into_iter().collect();
        let initial: HashSet<_> = initial.into_iter().collect();
        let labels: HashMap<_, _> = labels.into_iter().collect();
        for &s in initial.union(&labels.keys().cloned().collect()) {
            if s >= state_count {
                return Err(SimpleBuildError::StateOutOfRange {
                    state: s,
                    state_count,
                });
            }
        }
        let mut rows: Vec<HashMap<char, HashSet<SimpleLabeledNFAState>>> =
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
            labels,
            alphabet,
            transitions: rows,
        })
    }

    // TODO: docs
    pub fn map_labels<NewLabel: Hash + Eq + Clone>(
        &self,
        f: impl Fn(Label) -> NewLabel,
    ) -> SimpleLabeledNFA<NewLabel> {
        SimpleLabeledNFA {
            initial: self.initial.clone(),
            labels: self
                .labels
                .iter()
                .map(|(&k, v)| (k, f(v.clone())))
                .collect(),
            alphabet: self.alphabet.clone(),
            transitions: self.transitions.clone(),
        }
    }

    // TODO: docs
    pub fn drop_labels(&self) -> SimpleLabeledNFA<()> {
        self.map_labels(|_| ())
    }
}

impl<Label: Hash + Eq + Clone> LabeledAutomaton<Label> for SimpleLabeledNFA<Label> {
    type State = SimpleLabeledNFAState;
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

    fn get_label(&self, state: Self::State) -> Option<Label> {
        self.labels.get(&state).cloned()
    }
}

impl<Label: Hash + Eq + Clone> FiniteLabeledAutomaton<Label> for SimpleLabeledNFA<Label> {
    fn alphabet_set(&self) -> HashSet<Self::Input> {
        self.alphabet.clone()
    }
}

impl<Label: Hash + Eq + Clone> NonDeterministicLabeledAutomaton<Label> for SimpleLabeledNFA<Label> {
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a
    where
        Self::State: 'a,
    {
        self.initial.iter().copied()
    }

    fn successors<'a>(
        &'a self,
        state: Self::State,
        input: &Self::Input,
    ) -> impl Iterator<Item = Self::State> + 'a
    where
        Self::State: 'a,
    {
        self.transitions
            .get(state)
            .and_then(|row| row.get(input))
            .into_iter()
            .flat_map(|s| s.iter().copied())
    }
}

impl<Label: Hash + Eq + Clone> NonDeterministicFiniteLabeledAutomaton<Label>
    for SimpleLabeledNFA<Label>
{
    type CorrespondingDFA = SimpleLabeledDFA<Label>;

    fn to_dfa_by(&self, combine: impl Fn(Label, Label) -> Label) -> Self::CorrespondingDFA {
        self.to_simple_dfa(combine)
    }

    fn union(&self, other: &Self) -> Self {
        let na = self.transitions.len();
        let shift = na;

        let mut transitions = self.transitions.clone();
        for row in &other.transitions {
            let mut new_row: HashMap<char, HashSet<SimpleLabeledNFAState>> = HashMap::new();
            for (&a, tos) in row {
                let nt: HashSet<_> = tos.iter().map(|&p| p + shift).collect();
                new_row.insert(a, nt);
            }
            transitions.push(new_row);
        }

        let mut initial = self.initial.clone();
        initial.extend(other.initial.iter().map(|&s| s + shift));

        let mut labels = self.labels.clone();
        for (&s, l) in &other.labels {
            labels.insert(s + shift, l.clone());
        }

        let mut alphabet = self.alphabet.clone();
        alphabet.extend(&other.alphabet);

        Self {
            initial,
            labels,
            alphabet,
            transitions,
        }
    }
}

impl<Label: Hash + Eq + Clone> SimpleLabeledNFA<Label> {
    /// Subset construction. Each DFA state is an `NFA` state set; its label is
    /// the fold of all labels on member states using `combine`, in ascending
    /// NFA state order (deterministic when `combine` is not associative).
    fn to_simple_dfa(&self, combine: impl Fn(Label, Label) -> Label) -> SimpleLabeledDFA<Label> {
        let mut alphabet_vec: Vec<char> = self.alphabet.iter().copied().collect();
        alphabet_vec.sort_unstable();

        let start: BTreeSet<SimpleLabeledNFAState> = self.initial.iter().copied().collect();
        let mut subset_to_id: HashMap<BTreeSet<SimpleLabeledNFAState>, usize> = HashMap::new();
        let mut queue: VecDeque<BTreeSet<SimpleLabeledNFAState>> = VecDeque::new();
        subset_to_id.insert(start.clone(), 0);
        let mut next_id = 1usize;
        queue.push_back(start.clone());

        let mut dfa_labels: HashMap<usize, Label> = HashMap::new();
        if let Some(l) = Self::combined_label_for_subset(&start, &self.labels, &combine) {
            dfa_labels.insert(0, l);
        }

        let mut edges: Vec<(usize, char, usize)> = Vec::new();

        while let Some(sub) = queue.pop_front() {
            let sid = subset_to_id[&sub];
            for &a in &alphabet_vec {
                let mut dest: BTreeSet<SimpleLabeledNFAState> = BTreeSet::new();
                for &s in &sub {
                    if let Some(tos) = self.transitions.get(s).and_then(|row| row.get(&a)) {
                        dest.extend(tos.iter().copied());
                    }
                }
                let tid = if let Some(&id) = subset_to_id.get(&dest) {
                    id
                } else {
                    let id = next_id;
                    next_id += 1;
                    subset_to_id.insert(dest.clone(), id);
                    queue.push_back(dest.clone());
                    if let Some(l) = Self::combined_label_for_subset(&dest, &self.labels, &combine)
                    {
                        dfa_labels.insert(id, l);
                    }
                    id
                };
                edges.push((sid, a, tid));
            }
        }

        SimpleLabeledDFA::new_labeled_unchecked(
            next_id,
            0,
            dfa_labels,
            self.alphabet.iter().copied(),
            edges,
        )
    }

    fn combined_label_for_subset<F: Fn(Label, Label) -> Label>(
        subset: &BTreeSet<SimpleLabeledNFAState>,
        labels: &HashMap<SimpleLabeledNFAState, Label>,
        combine: &F,
    ) -> Option<Label> {
        let mut acc: Option<Label> = None;
        for &s in subset {
            if let Some(l) = labels.get(&s).cloned() {
                acc = Some(match acc {
                    None => l,
                    Some(a) => combine(a, l),
                });
            }
        }
        acc
    }
}
