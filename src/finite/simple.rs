#![allow(refining_impl_trait_reachable)]

use crate::finite::automaton::FiniteAutomaton;
use crate::finite::deterministic::DeterministicFiniteAutomaton;
use crate::finite::dot::ToDot;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::automaton::Automaton;
use crate::general::deterministic::DeterministicAutomaton;
use crate::general::nondeterministic::NonDeterministicAutomaton;

pub struct SimpleDFA;

pub struct SimpleNFA;

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
    fn to_nfa(&self) -> SimpleNFA {
        todo!("SimpleDFA::to_nfa")
    }

    fn minimize(&self) -> SimpleDFA {
        todo!("SimpleDFA::minimize")
    }

    fn accessible(&self) -> SimpleDFA {
        todo!("SimpleDFA::accessible")
    }

    fn co_accessible(&self) -> SimpleDFA {
        todo!("SimpleDFA::co_accessible")
    }

    fn trimmed(&self) -> SimpleDFA {
        todo!("SimpleDFA::trimmed")
    }

    fn complete(&self) -> SimpleDFA {
        todo!("SimpleDFA::complete")
    }
}

impl ToDot for SimpleDFA {}

impl Automaton for SimpleNFA {
    type State = usize;
    type Input = char;

    fn states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a {
        core::iter::empty() // TODO: SimpleNFA::states
    }

    fn alphabet<'a>(&'a self) -> impl Iterator<Item = Self::Input> + 'a {
        core::iter::empty() // TODO: SimpleNFA::alphabet
    }

    fn is_valid_state(&self, _state: Self::State) -> bool {
        todo!("SimpleNFA::is_valid_state")
    }

    fn is_initial_state(&self, _state: Self::State) -> bool {
        todo!("SimpleNFA::is_initial_state")
    }

    fn is_accepting_state(&self, _state: Self::State) -> bool {
        todo!("SimpleNFA::is_accepting_state")
    }
}

impl FiniteAutomaton for SimpleNFA {}

impl NonDeterministicAutomaton for SimpleNFA {
    fn initial_states<'a>(&'a self) -> impl Iterator<Item = Self::State> + 'a {
        core::iter::empty() // TODO: SimpleNFA::initial_states
    }

    fn successors<'a>(
        &'a self,
        _state: Self::State,
        _input: &Self::Input,
    ) -> impl Iterator<Item = Self::State> + 'a {
        core::iter::empty() // TODO: SimpleNFA::successors
    }
}

impl NonDeterministicFiniteAutomaton for SimpleNFA {
    fn to_dfa(&self) -> SimpleDFA {
        todo!("SimpleNFA::to_dfa")
    }

    fn union(&self, _other: &Self) -> SimpleNFA {
        todo!("SimpleNFA::union")
    }

    fn difference(&self, _other: &Self) -> SimpleNFA {
        todo!("SimpleNFA::difference")
    }

    fn concatenate(&self, _other: &Self) -> SimpleNFA {
        todo!("SimpleNFA::concatenate")
    }

    fn intersection(&self, _other: &Self) -> SimpleNFA {
        todo!("SimpleNFA::intersection")
    }

    fn star(&self) -> SimpleNFA {
        todo!("SimpleNFA::star")
    }

    fn reverse(&self) -> SimpleNFA {
        todo!("SimpleNFA::reverse")
    }

    fn complement(&self) -> SimpleNFA {
        todo!("SimpleNFA::complement")
    }

    fn accessible(&self) -> SimpleNFA {
        todo!("SimpleNFA::accessible")
    }

    fn co_accessible(&self) -> SimpleNFA {
        todo!("SimpleNFA::co_accessible")
    }

    fn trimmed(&self) -> SimpleNFA {
        todo!("SimpleNFA::trimmed")
    }
}

impl ToDot for SimpleNFA {}
