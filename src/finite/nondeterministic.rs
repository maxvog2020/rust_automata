use std::collections::{HashSet, VecDeque};

use crate::arbitrary::NonDeterministicAutomaton;
use crate::finite::automaton::FiniteAutomaton;
use crate::labeled::finite::DeterministicFiniteLabeledAutomaton;
use crate::labeled::finite::NonDeterministicFiniteLabeledAutomaton;
use crate::utility::clone_reduce;

/// Finite nondeterministic automata operations.
///
/// This trait adds finite-alphabet combinators and classical closure
/// operations for NFAs (union/intersection/concatenation/star, etc.),
/// together with determinization.
pub trait NonDeterministicFiniteAutomaton:
    NonDeterministicAutomaton + FiniteAutomaton + NonDeterministicFiniteLabeledAutomaton<()>
{
    /// Language difference: `L(self) \ L(other)`.
    fn difference(&self, other: &Self) -> Self;
    /// Concatenation: `L(self) · L(other)`.
    fn concatenate(&self, other: &Self) -> Self;
    /// Intersection: `L(self) ∩ L(other)`.
    fn intersection(&self, other: &Self) -> Self;

    /// Kleene star (0 or more repetitions).
    fn star(&self) -> Self;
    /// Reverse all transitions and swap initial/final roles.
    fn reverse(&self) -> Self;

    /// Trim to `accessible ∩ co_accessible`.
    fn trimmed(&self) -> Self;
    /// Complement language.
    fn complement(&self) -> Self;

    /// Restrict to `reachable` states.
    fn accessible(&self) -> Self;
    /// Restrict to `co-reachable` states.
    fn co_accessible(&self) -> Self;

    /// Determinize this NFA into a DFA (subset construction).
    fn to_dfa(&self) -> Self::CorrespondingDFA {
        self.to_dfa_by(|_, _| ())
    }

    /// Determinize this NFA into a minimized DFA.
    fn to_minimized_dfa(&self) -> Self::CorrespondingDFA {
        self.to_dfa().minimize()
    }

    /// Concatenation across many NFAs.
    ///
    /// Computes `L(a0) · L(a1) · ...` in iteration order.
    ///
    /// Returns `None` if the slice is empty.
    fn concatenate_all(automata: &[Self]) -> Option<Self>
    where
        Self: Clone,
    {
        clone_reduce(automata, |a, b| a.concatenate(b))
    }

    /// Language intersection across many NFAs.
    ///
    /// Computes `L(a0) ∩ L(a1) ∩ ...` for every automaton produced by `automata`.
    ///
    /// Returns `None` if the slice is empty.
    fn intersect_all(automata: &[Self]) -> Option<Self>
    where
        Self: Clone,
    {
        clone_reduce(automata, |a, b| a.intersection(b))
    }

    /// Accepting states of `self` that can be reached **in sync** with some
    /// accepting state of `other`: both follow the same input word, using only
    /// symbols that appear in both alphabets.
    fn accepting_states_compatible_with(&self, other: &Self) -> HashSet<Self::State> {
        let mut common = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        let common_alphabet = self.common_alphabet(other);

        for initial_state1 in self.initial_states() {
            for initial_state2 in other.initial_states() {
                queue.push_back((initial_state1, initial_state2));
            }
        }

        while let Some((state1, state2)) = queue.pop_front() {
            if visited.contains(&(state1, state2)) {
                continue;
            }

            visited.insert((state1, state2));

            if self.is_accepting_state(state1) && other.is_accepting_state(state2) {
                common.insert(state1);
            }

            for input in &common_alphabet {
                for new_state1 in self.successors(state1, input) {
                    for new_state2 in other.successors(state2, input) {
                        queue.push_back((new_state1, new_state2));
                    }
                }
            }
        }

        common
    }

    /// Whether the recognized language is empty.
    fn is_empty_language(&self) -> bool {
        !self
            .reachable_states_set()
            .iter()
            .any(|&s| self.is_accepting_state(s))
    }

    /// Check whether `L(self) ⊆ L(other)`.
    fn is_subset_of(&self, other: &Self) -> bool {
        self.difference(other).is_empty_language()
    }

    /// Check whether `L(self) = L(other)`.
    fn is_equivalent_to(&self, other: &Self) -> bool {
        self.is_subset_of(other) && other.is_subset_of(self)
    }
}
