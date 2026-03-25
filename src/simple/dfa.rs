use crate::finite::automaton::FiniteAutomaton;
use crate::finite::deterministic::DeterministicFiniteAutomaton;
use crate::finite::dot::ToDot;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::automaton::Automaton;
use crate::general::deterministic::DeterministicAutomaton;

pub struct SimpleDFA;

impl Automaton for SimpleDFA {
    type State = usize;
    type Input = char;

    fn states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a {
        core::iter::empty() // TODO: SimpleDFA::states
    }

    fn alphabet<'a>(&'a self) -> impl Iterator<Item = Self::Input> + 'a {
        core::iter::empty() // TODO: SimpleDFA::alphabet
    }

    fn is_valid_state(&self, _state: Self::State) -> bool {
        todo!("SimpleDFA::is_valid_state")
    }

    fn is_initial_state(&self, _state: Self::State) -> bool {
        todo!("SimpleDFA::is_initial_state")
    }

    fn is_accepting_state(&self, _state: Self::State) -> bool {
        todo!("SimpleDFA::is_accepting_state")
    }
}

impl FiniteAutomaton for SimpleDFA {}

impl DeterministicAutomaton for SimpleDFA {
    fn initial_state(&self) -> Self::State {
        todo!("SimpleDFA::initial_state")
    }

    fn transition(&self, _state: Self::State, _input: &Self::Input) -> Option<Self::State> {
        todo!("SimpleDFA::transition")
    }
}

impl DeterministicFiniteAutomaton for SimpleDFA {
    fn to_nfa(&self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleDFA::to_nfa")
    }

    fn minimize(&self) -> impl DeterministicFiniteAutomaton {
        todo!("SimpleDFA::minimize")
    }

    fn complete(&self) -> impl DeterministicFiniteAutomaton {
        todo!("SimpleDFA::complete")
    }
}

impl ToDot for SimpleDFA {}
