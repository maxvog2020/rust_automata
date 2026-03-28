use crate::arbitrary::automaton::Automaton;
use crate::labeled::arbitrary::DeterministicLabeledAutomaton;

/// Deterministic automaton semantics.
///
/// Implementors provide a single `transition` function. This trait then
/// supplies [`accepts`](DeterministicAutomaton::accepts) as a word-level convenience.
pub trait DeterministicAutomaton: Automaton + DeterministicLabeledAutomaton<()> {
    /// Whether `word` is accepted: run from [`initial_state`](DeterministicLabeledAutomaton::initial_state)
    /// and require a defined transition for every symbol, ending in a state
    /// with [`Some`](Option::Some) label (accepting).
    fn accepts(&self, word: &[Self::Input]) -> bool {
        self.get_label_of_word(word).is_some()
    }
}
