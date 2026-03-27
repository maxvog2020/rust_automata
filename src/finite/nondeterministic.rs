use crate::finite::automaton::FiniteAutomaton;
use crate::finite::deterministic::DeterministicFiniteAutomaton;
use crate::general::NonDeterministicAutomaton;

/// Finite nondeterministic automata operations.
///
/// This trait adds finite-alphabet combinators and classical closure
/// operations for NFAs (union/intersection/concatenation/star, etc.),
/// together with determinization.
pub trait NonDeterministicFiniteAutomaton: NonDeterministicAutomaton + FiniteAutomaton {
    /// Deterministic representation obtained by determinization.
    type CorrespondingDFA: DeterministicFiniteAutomaton<State = Self::State, Input = Self::Input, CorrespondingNFA = Self>;

    /// Determinize this NFA into a DFA (subset construction).
    fn to_dfa(&self) -> Self::CorrespondingDFA;

    /// Language union: `L(self) ∪ L(other)`.
    fn union(&self, other: &Self) -> Self;
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
    ///
    /// Typically requires a total DFA completion pipeline internally.
    fn complement(&self) -> Self;

    /// Restrict to `reachable` states.
    fn accessible(&self) -> Self;
    /// Restrict to `co-reachable` states.
    fn co_accessible(&self) -> Self;

    /// Check whether `L(self) ⊆ L(other)`.
    fn is_subset_of(&self, other: &Self) -> bool;
    /// Check whether `L(self) = L(other)`.
    fn is_equivalent_to(&self, other: &Self) -> bool;

    /// Whether the recognized language is empty.
    fn is_empty_language(&self) -> bool {
        !self.reachable_states().iter().any(|&s| self.is_accepting_state(s))
    }

    /// Language union across many NFAs.
    ///
    /// Computes `L(a0) ∪ L(a1) ∪ ...` for every automaton produced by `automata`.
    ///
    /// Returns `None` if the slice is empty.
    fn union_all(automata: &[Self]) -> Option<Self>
        where Self: Clone + Sized
    {
        clone_reduce(automata, |a, b| a.union(b))
    }
    
    /// Concatenation across many NFAs.
    ///
    /// Computes `L(a0) · L(a1) · ...` in iteration order.
    ///
    /// Returns `None` if the slice is empty.
    fn concatenate_all(automata: &[Self]) -> Option<Self> 
        where Self: Clone + Sized 
    {
        clone_reduce(automata, |a, b| a.concatenate(b))
    }
    
    /// Language intersection across many NFAs.
    ///
    /// Computes `L(a0) ∩ L(a1) ∩ ...` for every automaton produced by `automata`.
    ///
    /// Returns `None` if the slice is empty.
    fn intersect_all(automata: &[Self]) -> Option<Self> 
        where Self: Clone + Sized 
    {
        clone_reduce(automata, |a, b| a.intersection(b))
    }
}

fn clone_reduce<T: Clone>(arr: &[T], f: impl Fn(T, &T) -> T) -> Option<T> {
    let mut iter = arr.iter();
    let item = iter.next()?;
    Some(iter.fold(item.clone(), f))
}

