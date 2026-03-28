use std::collections::HashSet;
use std::hash::Hash;

use crate::labeled::arbitrary::LabeledAutomaton;

/// [`LabeledAutomaton`] whose states and alphabet can be collected into sets.
pub trait FiniteLabeledAutomaton<Label: Eq + Clone>: LabeledAutomaton<Label> + Sized {
    /// Return the automaton's alphabet as a set.
    fn alphabet_set(&self) -> HashSet<Self::Input> {
        self.alphabet().collect()
    }

    /// Return the automaton's states as a set.
    fn states_set(&self) -> HashSet<Self::State> {
        self.states().collect()
    }

    // Return the automaton's labels as a set.
    fn labels_set(&self) -> HashSet<Label>
    where
        Label: Hash,
    {
        self.labels().collect()
    }

    /// The set of symbols shared with `other`.
    fn common_alphabet(&self, other: &Self) -> HashSet<Self::Input> {
        let alphabet1: HashSet<Self::Input> = self.alphabet_set();
        let alphabet2: HashSet<Self::Input> = other.alphabet_set();
        alphabet1.intersection(&alphabet2).cloned().collect()
    }
}
