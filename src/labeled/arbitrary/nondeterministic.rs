use std::hash::Hash;

use crate::labeled::arbitrary::automaton::LabeledAutomaton;

// TODO: docs
pub trait NonDeterministicLabeledAutomaton<Label: Hash + Eq + Clone>:
    LabeledAutomaton<Label>
{
    /// Iterator over initial states.
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a
    where
        Self::State: 'a;

    /// Successors of `state` under `input`.
    fn successors<'a>(
        &'a self,
        state: Self::State,
        input: &Self::Input,
    ) -> impl Iterator<Item = Self::State> + 'a
    where
        Self::State: 'a;
}
