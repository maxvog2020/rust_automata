use crate::finite::automaton::FiniteAutomaton;
use crate::finite::dot::ToDot;
use crate::finite::nondeterministic::NonDeterministicFiniteAutomaton;
use crate::general::automaton::Automaton;
use crate::general::nondeterministic::NonDeterministicAutomaton;

pub struct SimpleNFA;

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

    fn successors<'a>(&'a self, _state: Self::State, _input: &Self::Input) -> impl Iterator<Item = Self::State> + 'a {
        core::iter::empty() // TODO: SimpleNFA::successors
    }
}

impl NonDeterministicFiniteAutomaton for SimpleNFA {
    fn to_dfa(&self) -> impl crate::finite::deterministic::DeterministicFiniteAutomaton {
        todo!("SimpleNFA::to_dfa")
    }
    
    fn union(&self, other: &Self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::union")
    }
    
    fn difference(&self, other: &Self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::difference")
    }
    
    fn concatenate(&self, other: &Self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::concatenate")
    }
    
    fn intersection(&self, other: &Self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::intersection")
    }
    
    fn star(&self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::star")
    }
    
    fn reverse(&self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::reverse")
    }
    
    fn trimmed(&self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::trimmed")
    }
    
    fn complement(&self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::complement")
    }
    
    fn accessible(&self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::accessible")
    }
    
    fn co_accessible(&self) -> impl NonDeterministicFiniteAutomaton {
        todo!("SimpleNFA::co_accessible")
    }

    fn is_subset_of(&self, _other: &Self) -> bool {
        todo!("SimpleNFA::is_subset_of")
    }
    
    fn is_equivalent_to(&self, _other: &Self) -> bool {
        todo!("SimpleNFA::is_equivalent_to")
    }
}

impl ToDot for SimpleNFA {}
