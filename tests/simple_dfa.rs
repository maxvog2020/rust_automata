#[path = "common/mod.rs"]
mod common;

use automata::general::automaton::Automaton;
use automata::general::deterministic::DeterministicAutomaton;
use automata::simple::SimpleDFA;

use common::{accepts_dfa, accepts_nfa, word_a, word_repeat};

fn dfa_even_len() -> SimpleDFA {
    let alphabet = ['a'];
    let edges = [(0usize, 'a', 1usize), (1usize, 'a', 0usize)];
    SimpleDFA::try_new(2, 0, [0], alphabet, edges).unwrap()
}

fn dfa_incomplete_len1_only() -> SimpleDFA {
    let alphabet = ['a'];
    let edges = [(0usize, 'a', 1usize)];
    SimpleDFA::try_new(2, 0, [1], alphabet, edges).unwrap()
}

#[test]
fn dfa_even_len_accepts_len_0() {
    let dfa = dfa_even_len();
    assert!(dfa.accepts(&[]));
}

#[test]
fn dfa_even_len_accepts_len_1() {
    let dfa = dfa_even_len();
    let w = word_a(1);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_2() {
    let dfa = dfa_even_len();
    let w = word_a(2);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_3() {
    let dfa = dfa_even_len();
    let w = word_a(3);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_4() {
    let dfa = dfa_even_len();
    let w = word_a(4);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_5() {
    let dfa = dfa_even_len();
    let w = word_a(5);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_6() {
    let dfa = dfa_even_len();
    let w = word_a(6);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_7() {
    let dfa = dfa_even_len();
    let w = word_a(7);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_8() {
    let dfa = dfa_even_len();
    let w = word_a(8);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_9() {
    let dfa = dfa_even_len();
    let w = word_a(9);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_10() {
    let dfa = dfa_even_len();
    let w = word_a(10);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_11() {
    let dfa = dfa_even_len();
    let w = word_a(11);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_12() {
    let dfa = dfa_even_len();
    let w = word_a(12);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_13() {
    let dfa = dfa_even_len();
    let w = word_a(13);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_14() {
    let dfa = dfa_even_len();
    let w = word_a(14);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_15() {
    let dfa = dfa_even_len();
    let w = word_a(15);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_16() {
    let dfa = dfa_even_len();
    let w = word_a(16);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_even_len_accepts_len_17() {
    let dfa = dfa_even_len();
    let w = word_a(17);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_incomplete_len1_accepts_len_0() {
    let dfa = dfa_incomplete_len1_only();
    let w = word_a(0);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_incomplete_len1_accepts_len_1() {
    let dfa = dfa_incomplete_len1_only();
    let w = word_a(1);
    assert!(dfa.accepts(&w));
}

#[test]
fn dfa_incomplete_len1_accepts_len_2() {
    let dfa = dfa_incomplete_len1_only();
    let w = word_a(2);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_incomplete_len1_accepts_len_3() {
    let dfa = dfa_incomplete_len1_only();
    let w = word_a(3);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_incomplete_len1_accepts_len_4() {
    let dfa = dfa_incomplete_len1_only();
    let w = word_a(4);
    assert!(!dfa.accepts(&w));
}

#[test]
fn dfa_complete_preserves_incomplete_len1_accepts_len_0() {
    let dfa = dfa_incomplete_len1_only();
    let dfa_c = dfa.complete_copy();
    let a = dfa_c.alphabet().next().unwrap();
    let w = word_repeat(a, 0);
    assert!(!accepts_dfa(&dfa_c, &w));
}

#[test]
fn dfa_complete_preserves_incomplete_len1_accepts_len_1() {
    let dfa = dfa_incomplete_len1_only();
    let dfa_c = dfa.complete_copy();
    let a = dfa_c.alphabet().next().unwrap();
    let w = word_repeat(a, 1);
    assert!(accepts_dfa(&dfa_c, &w));
}

#[test]
fn dfa_complete_preserves_incomplete_len1_accepts_len_2() {
    let dfa = dfa_incomplete_len1_only();
    let dfa_c = dfa.complete_copy();
    let a = dfa_c.alphabet().next().unwrap();
    let w = word_repeat(a, 2);
    assert!(!accepts_dfa(&dfa_c, &w));
}

#[test]
fn dfa_complete_preserves_incomplete_len1_accepts_len_3() {
    let dfa = dfa_incomplete_len1_only();
    let dfa_c = dfa.complete_copy();
    let a = dfa_c.alphabet().next().unwrap();
    let w = word_repeat(a, 3);
    assert!(!accepts_dfa(&dfa_c, &w));
}

#[test]
fn dfa_complete_preserves_incomplete_len1_accepts_len_4() {
    let dfa = dfa_incomplete_len1_only();
    let dfa_c = dfa.complete_copy();
    let a = dfa_c.alphabet().next().unwrap();
    let w = word_repeat(a, 4);
    assert!(!accepts_dfa(&dfa_c, &w));
}

#[test]
fn dfa_to_nfa_preserves_even_len_language() {
    let dfa = dfa_even_len();
    let nfa = dfa.to_simple_nfa_copy();
    let a = nfa.alphabet().next().unwrap();
    for len in 0..10 {
        let expect = len % 2 == 0;
        let w = word_repeat(a, len);
        assert_eq!(accepts_nfa(&nfa, &w), expect);
    }
}

#[test]
fn dfa_minimize_reduces_and_preserves_even_len_language() {
    let alphabet = ['a'];
    let edges = [
        (0usize, 'a', 2usize),
        (1usize, 'a', 3usize),
        (2usize, 'a', 0usize),
        (3usize, 'a', 1usize),
    ];
    let dfa = SimpleDFA::try_new(4, 0, [0, 1], alphabet, edges).unwrap();

    let min = dfa.minimize_copy();
    assert_eq!(min.states().count(), 2);
    let a = min.alphabet().next().unwrap();

    for len in 0..12 {
        let w = word_repeat(a, len);
        assert_eq!(accepts_dfa(&min, &w), len % 2 == 0);
    }
}
