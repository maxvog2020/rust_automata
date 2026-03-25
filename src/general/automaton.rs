use std::hash::Hash;

pub trait Automaton {
    type State: Hash + Eq + Copy;
    type Input: Hash + Eq + Clone;

    fn states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a;
    fn alphabet<'a>(&'a self) -> impl Iterator<Item = Self::Input> + 'a;

    fn is_valid_state(&self, state: Self::State) -> bool;
    fn is_initial_state(&self, state: Self::State) -> bool;
    fn is_accepting_state(&self, state: Self::State) -> bool;
}
