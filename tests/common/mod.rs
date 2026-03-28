use std::iter::repeat_n;

use automata_core::arbitrary::DeterministicAutomaton;
use automata_core::finite::NonDeterministicFiniteAutomaton;
use automata_core::simple::{SimpleDFA, SimpleNFA};

pub fn word_repeat<I: Clone>(sym: I, len: usize) -> Vec<I> {
    repeat_n(sym, len).collect()
}

pub fn word_a(len: usize) -> Vec<char> {
    vec!['a'; len]
}

pub fn accepts_nfa(nfa: &SimpleNFA, word: &[char]) -> bool {
    nfa.to_dfa().accepts(word)
}

pub fn accepts_dfa(dfa: &SimpleDFA, word: &[char]) -> bool {
    dfa.accepts(word)
}
