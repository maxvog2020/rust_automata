use crate::arbitrary::automaton::Automaton;
use crate::labeled::arbitrary::DeterministicLabeledAutomaton;

/// Deterministic automaton semantics.
///
/// Implementors provide a single `transition` function. This trait then
/// supplies [`accepts`] as a word-level convenience.
pub trait DeterministicAutomaton: Automaton + DeterministicLabeledAutomaton<()> {
    // TODO: docs
    fn accepts(&self, word: &[Self::Input]) -> bool {
        self.get_label_of_word(word).is_some()
    }
}
