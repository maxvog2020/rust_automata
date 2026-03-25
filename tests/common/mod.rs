use std::collections::HashSet;

use automata::general::automaton::Automaton;
use automata::general::deterministic::DeterministicAutomaton;
use automata::general::nondeterministic::NonDeterministicAutomaton;
use automata::simple::{SimpleDFA, SimpleNFA};

pub fn word_repeat<I: Clone>(sym: I, len: usize) -> Vec<I> {
    std::iter::repeat_n(sym, len).collect()
}

pub fn word_a(len: usize) -> Vec<char> {
    vec!['a'; len]
}

pub fn accepts_nfa(nfa: &SimpleNFA, word: &[char]) -> bool {
    let mut current: HashSet<_> = nfa.initial_states().collect();
    for ch in word {
        let mut next = HashSet::new();
        for &s in &current {
            next.extend(nfa.successors(s, ch));
        }
        current = next;
        if current.is_empty() {
            return false;
        }
    }
    current.iter().copied().any(|s| nfa.is_accepting_state(s))
}

pub fn accepts_dfa(dfa: &SimpleDFA, word: &[char]) -> bool {
    dfa.accepts(word)
}
