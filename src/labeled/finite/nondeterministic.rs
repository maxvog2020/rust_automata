use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use crate::labeled::arbitrary::NonDeterministicLabeledAutomaton;
use crate::labeled::finite::automaton::FiniteLabeledAutomaton;
use crate::labeled::finite::deterministic::DeterministicFiniteLabeledAutomaton;
use crate::utility::clone_reduce;

// TODO: docs
pub trait NonDeterministicFiniteLabeledAutomaton<Label: Hash + Eq + Clone>: NonDeterministicLabeledAutomaton<Label> + FiniteLabeledAutomaton<Label> {
    /// Deterministic representation obtained by determinization.
    type CorrespondingDFA: DeterministicFiniteLabeledAutomaton<
            Label,
            State = Self::State,
            Input = Self::Input,
            CorrespondingNFA = Self,
        >;

    /// Determinize this NFA into a DFA (subset construction).
    fn to_dfa_by(&self, combine: impl Fn(Label, Label) -> Label) -> Self::CorrespondingDFA;

    // TODO: docs
    fn union(&self, other: &Self) -> Self;

    /// Language union across many NFAs.
    ///
    /// Computes `L(a0) ∪ L(a1) ∪ ...` for every automaton produced by `automata`.
    ///
    /// Returns `None` if the slice is empty.
    fn union_all(automata: &[Self]) -> Option<Self>
    where
        Self: Clone,
    {
        clone_reduce(automata, |a, b| a.union(b))
    }

    /// All states reachable from the initial states.
    ///
    /// This helper explores the automaton by iterating successor transitions
    /// over every symbol in `alphabet()`.
    fn reachable_states_set(&self) -> HashSet<Self::State> {
        let mut reachable = HashSet::new();
        let mut queue = VecDeque::new();

        for initial_state in self.initial_states() {
            queue.push_back(initial_state);
        }

        while let Some(state) = queue.pop_front() {
            if reachable.contains(&state) {
                continue;
            }

            reachable.insert(state);

            for input in self.alphabet() {
                for successor in self.successors(state, &input) {
                    queue.push_back(successor);
                }
            }
        }

        reachable
    }
}

