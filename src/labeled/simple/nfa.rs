use std::collections::{HashMap, HashSet};
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
    pub fn map_labels<NewLabel: Hash + Eq + Clone>(&self, f: impl Fn(Label) -> NewLabel) -> SimpleLabeledNFA<NewLabel> {
        SimpleLabeledNFA {
            initial: self.initial.clone(),
            labels: self.labels.iter().map(|(&k, v)| (k, f(v.clone()))).collect(),
            alphabet: self.alphabet.clone(),
            transitions: self.transitions.clone(),
        }
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

impl<Label: Hash + Eq + Clone> NonDeterministicFiniteLabeledAutomaton<Label> for SimpleLabeledNFA<Label> {
    type CorrespondingDFA = SimpleLabeledDFA<Label>;

    fn to_dfa_by(&self, _combine: impl Fn(Label, Label) -> Label) -> Self::CorrespondingDFA {
        self.to_simple_dfa()
    }
    
    fn union(&self, _other: &Self) -> Self {
        todo!()
    }
}

impl<Label: Hash + Eq + Clone> SimpleLabeledNFA<Label> {
    fn to_simple_dfa(&self) -> SimpleLabeledDFA<Label> {
        todo!()
        // let mut alphabet_vec: Vec<char> = self.alphabet.iter().copied().collect();
        // alphabet_vec.sort_unstable();
        // let start: BTreeSet<SimpleLabeledNFAState> = self.initial.iter().copied().collect();
        // let mut subset_to_id: HashMap<BTreeSet<SimpleLabeledNFAState>, usize> = HashMap::new();
        // let mut queue: VecDeque<BTreeSet<SimpleLabeledNFAState>> = VecDeque::new();
        // subset_to_id.insert(start.clone(), 0);
        // let mut next_id = 1usize;
        // queue.push_back(start);

        // let mut trans_out: HashMap<(usize, char), usize> = HashMap::new();

        // while let Some(sub) = queue.pop_front() {
        //     let sid = subset_to_id[&sub];
        //     for &a in &alphabet_vec {
        //         let mut dest: BTreeSet<SimpleLabeledNFAState> = BTreeSet::new();
        //         for &s in &sub {
        //             if let Some(tos) = self.transitions[s].get(&a) {
        //                 dest.extend(tos.iter().copied());
        //             }
        //         }
        //         let tid = if let Some(&id) = subset_to_id.get(&dest) {
        //             id
        //         } else {
        //             let id = next_id;
        //             next_id += 1;
        //             subset_to_id.insert(dest.clone(), id);
        //             queue.push_back(dest);
        //             id
        //         };
        //         trans_out.insert((sid, a), tid);
        //     }
        // }

        // let num_states = next_id;
        // let accepting_dfa: HashSet<usize> = subset_to_id
        //     .iter()
        //     .filter(|(set, _)| set.iter().any(|s| self.accepting.contains(s)))
        //     .map(|(_, id)| *id)
        //     .collect();

        // let edges: Vec<(usize, char, usize)> =
        //     trans_out.into_iter().map(|((q, a), p)| (q, a, p)).collect();

        // SimpleLabeledDFA::new_unchecked(
        //     num_states,
        //     0,
        //     accepting_dfa,
        //     self.alphabet.iter().copied(),
        //     edges,
        // )
    }
}