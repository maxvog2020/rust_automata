#[path = "common/mod.rs"]
mod common;

use std::collections::HashSet;

use automata_core::arbitrary::DeterministicAutomaton;
use automata_core::labeled::arbitrary::LabeledAutomaton;
use automata_core::labeled::arbitrary::NonDeterministicLabeledAutomaton;
use automata_core::labeled::finite::NonDeterministicFiniteLabeledAutomaton;
use automata_core::simple::SimpleNFA;
use automata_core::{finite::NonDeterministicFiniteAutomaton, simple::SimpleBuildError};

use common::{accepts_dfa, accepts_nfa, word_a, word_repeat};

////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////

fn chars(s: &str) -> Vec<char> {
    s.chars().collect()
}

fn reverse_word(w: &[char]) -> Vec<char> {
    w.iter().rev().copied().collect()
}

fn nfa_even_len() -> SimpleNFA {
    let alphabet = ['a'];
    let edges = [(0usize, 'a', 1usize), (1usize, 'a', 0usize)];
    SimpleNFA::try_new(2, [0], [0], alphabet, edges).unwrap()
}

fn nfa_odd_len() -> SimpleNFA {
    let alphabet = ['a'];
    let edges = [(0usize, 'a', 1usize), (1usize, 'a', 0usize)];
    SimpleNFA::try_new(2, [0], [1], alphabet, edges).unwrap()
}

/// Accepts exactly the single word `word` (as a path 0 → 1 → … → n).
fn nfa_literal(word: &[char]) -> SimpleNFA {
    let n = word.len();
    let alphabet: HashSet<char> = word.iter().copied().collect();
    let edges: Vec<(usize, char, usize)> = (0..n).map(|i| (i, word[i], i + 1)).collect();
    SimpleNFA::try_new(n + 1, [0], [n], alphabet, edges).unwrap()
}

////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////

#[test]
fn nfa_even_len_accepts_len_0() {
    let nfa = nfa_even_len();
    let w = word_a(0);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_1() {
    let nfa = nfa_even_len();
    let w = word_a(1);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_2() {
    let nfa = nfa_even_len();
    let w = word_a(2);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_3() {
    let nfa = nfa_even_len();
    let w = word_a(3);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_4() {
    let nfa = nfa_even_len();
    let w = word_a(4);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_5() {
    let nfa = nfa_even_len();
    let w = word_a(5);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_6() {
    let nfa = nfa_even_len();
    let w = word_a(6);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_7() {
    let nfa = nfa_even_len();
    let w = word_a(7);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_8() {
    let nfa = nfa_even_len();
    let w = word_a(8);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_even_len_accepts_len_9() {
    let nfa = nfa_even_len();
    let w = word_a(9);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_0() {
    let nfa = nfa_odd_len();
    let w = word_a(0);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_1() {
    let nfa = nfa_odd_len();
    let w = word_a(1);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_2() {
    let nfa = nfa_odd_len();
    let w = word_a(2);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_3() {
    let nfa = nfa_odd_len();
    let w = word_a(3);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_4() {
    let nfa = nfa_odd_len();
    let w = word_a(4);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_5() {
    let nfa = nfa_odd_len();
    let w = word_a(5);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_6() {
    let nfa = nfa_odd_len();
    let w = word_a(6);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_7() {
    let nfa = nfa_odd_len();
    let w = word_a(7);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_8() {
    let nfa = nfa_odd_len();
    let w = word_a(8);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_odd_len_accepts_len_9() {
    let nfa = nfa_odd_len();
    let w = word_a(9);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn nfa_complement_even_accepts_len_0() {
    let even = nfa_even_len();
    let comp = even.complement();
    let a = comp.alphabet().next().unwrap();
    let w = word_repeat(a, 0);
    assert!(!accepts_nfa(&comp, &w));
}

#[test]
fn nfa_complement_even_accepts_len_1() {
    let even = nfa_even_len();
    let comp = even.complement();
    let a = comp.alphabet().next().unwrap();
    let w = word_repeat(a, 1);
    assert!(accepts_nfa(&comp, &w));
}

#[test]
fn nfa_complement_even_accepts_len_2() {
    let even = nfa_even_len();
    let comp = even.complement();
    let a = comp.alphabet().next().unwrap();
    let w = word_repeat(a, 2);
    assert!(!accepts_nfa(&comp, &w));
}

#[test]
fn nfa_complement_even_accepts_len_3() {
    let even = nfa_even_len();
    let comp = even.complement();
    let a = comp.alphabet().next().unwrap();
    let w = word_repeat(a, 3);
    assert!(accepts_nfa(&comp, &w));
}

#[test]
fn nfa_union_even_odd_accepts_len_0() {
    let even = nfa_even_len();
    let odd = nfa_odd_len();
    let u = even.union(&odd);
    let a = u.alphabet().next().unwrap();
    let w = word_repeat(a, 0);
    assert!(accepts_nfa(&u, &w));
}

#[test]
fn nfa_union_even_odd_accepts_len_1() {
    let even = nfa_even_len();
    let odd = nfa_odd_len();
    let u = even.union(&odd);
    let a = u.alphabet().next().unwrap();
    let w = word_repeat(a, 1);
    assert!(accepts_nfa(&u, &w));
}

#[test]
fn nfa_union_even_odd_accepts_len_2() {
    let even = nfa_even_len();
    let odd = nfa_odd_len();
    let u = even.union(&odd);
    let a = u.alphabet().next().unwrap();
    let w = word_repeat(a, 2);
    assert!(accepts_nfa(&u, &w));
}

#[test]
fn nfa_intersection_even_odd_rejects_len_0() {
    let even = nfa_even_len();
    let odd = nfa_odd_len();
    let i = even.intersection(&odd);
    let a = i.alphabet().next().unwrap();
    let w = word_repeat(a, 0);
    assert!(!accepts_nfa(&i, &w));
}

#[test]
fn nfa_intersection_even_odd_rejects_len_1() {
    let even = nfa_even_len();
    let odd = nfa_odd_len();
    let i = even.intersection(&odd);
    let a = i.alphabet().next().unwrap();
    let w = word_repeat(a, 1);
    assert!(!accepts_nfa(&i, &w));
}

#[test]
fn nfa_intersection_even_odd_rejects_len_2() {
    let even = nfa_even_len();
    let odd = nfa_odd_len();
    let i = even.intersection(&odd);
    let a = i.alphabet().next().unwrap();
    let w = word_repeat(a, 2);
    assert!(!accepts_nfa(&i, &w));
}

#[test]
fn nfa_star_odd_accepts_len_0() {
    let odd = nfa_odd_len();
    let s = odd.star();
    let a = s.alphabet().next().unwrap();
    let w = word_repeat(a, 0);
    assert!(accepts_nfa(&s, &w));
}

#[test]
fn nfa_star_odd_accepts_len_2() {
    let odd = nfa_odd_len();
    let s = odd.star();
    let a = s.alphabet().next().unwrap();
    let w = word_repeat(a, 2);
    assert!(accepts_nfa(&s, &w));
}

#[test]
fn nfa_concat_even_odd_accepts_len_1() {
    let even = nfa_even_len();
    let odd = nfa_odd_len();
    let c = even.concatenate(&odd);
    let a = c.alphabet().next().unwrap();
    let w = word_repeat(a, 1);
    assert!(accepts_nfa(&c, &w));
}

#[test]
fn nfa_concat_even_odd_accepts_len_3() {
    let even = nfa_even_len();
    let odd = nfa_odd_len();
    let c = even.concatenate(&odd);
    let a = c.alphabet().next().unwrap();
    let w = word_repeat(a, 3);
    assert!(accepts_nfa(&c, &w));
}

// -----------------------
// Complex: abaaacc / reverse, chained ops
// -----------------------

#[test]
fn complex_literal_abaaacc_accepted() {
    let w = chars("abaaacc");
    let nfa = nfa_literal(&w);
    assert!(accepts_nfa(&nfa, &w));
}

#[test]
fn complex_literal_abaaacc_rejects_prefix() {
    let w = chars("abaaacc");
    let bad = chars("abaaac");
    let nfa = nfa_literal(&w);
    assert!(!accepts_nfa(&nfa, &bad));
}

#[test]
fn complex_literal_reverse_ccaaaba_accepted() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    assert_eq!(r.iter().collect::<String>(), "ccaaaba");
    let nfa = nfa_literal(&r);
    assert!(accepts_nfa(&nfa, &r));
}

#[test]
fn complex_literal_reverse_rejects_forward_word() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let nfa = nfa_literal(&r);
    assert!(!accepts_nfa(&nfa, &w));
}

#[test]
fn complex_union_forward_and_reverse_accepts_both() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let n = nfa_literal(&w).union(&nfa_literal(&r));
    assert!(accepts_nfa(&n, &w));
    assert!(accepts_nfa(&n, &r));
}

#[test]
fn complex_union_then_reverse_swaps_membership() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let n = nfa_literal(&w).union(&nfa_literal(&r));
    let rev = n.reverse();
    assert!(accepts_nfa(&rev, &w));
    assert!(accepts_nfa(&rev, &r));
}

#[test]
fn complex_double_reverse_restores_singleton_language() {
    let w = chars("abaaacc");
    let n = nfa_literal(&w);
    let n2 = n.reverse().reverse();
    assert!(accepts_nfa(&n2, &w));
    assert!(!accepts_nfa(&n2, &chars("abaaaca")));
}

#[test]
fn complex_concat_forward_then_reverse_is_palindrome_pair() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let c = nfa_literal(&w).concatenate(&nfa_literal(&r));
    let mut wr = w.clone();
    wr.extend(r.iter().copied());
    assert!(accepts_nfa(&c, &wr));
    assert!(!accepts_nfa(&c, &w));
}

#[test]
fn complex_star_of_union_allows_iteration() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let base = nfa_literal(&w).union(&nfa_literal(&r));
    let s = base.star();
    assert!(accepts_nfa(&s, &[]));
    assert!(accepts_nfa(&s, &w));
    assert!(accepts_nfa(&s, &r));
    let mut ww = w.clone();
    ww.extend(w.iter().copied());
    assert!(accepts_nfa(&s, &ww));
}

#[test]
fn complex_nested_reverse_union_star_chain() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let chain = nfa_literal(&w)
        .reverse()
        .union(&nfa_literal(&r).reverse())
        .star()
        .reverse()
        .union(&nfa_literal(&w));
    assert!(accepts_nfa(&chain, &w));
    assert!(accepts_nfa(&chain, &r));
}

#[test]
fn complex_intersection_union_with_literal_is_singleton() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let u = nfa_literal(&w).union(&nfa_literal(&r));
    let i = nfa_literal(&w).intersection(&u);
    assert!(accepts_nfa(&i, &w));
    assert!(!accepts_nfa(&i, &r));
}

#[test]
fn complex_difference_removes_reverse_from_union() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let u = nfa_literal(&w).union(&nfa_literal(&r));
    let d = u.difference(&nfa_literal(&r));
    assert!(accepts_nfa(&d, &w));
    assert!(!accepts_nfa(&d, &r));
}

#[test]
fn complex_trimmed_literal_same_language() {
    let w = chars("abaaacc");
    let n = nfa_literal(&w);
    let t = n.trimmed();
    assert!(accepts_nfa(&t, &w));
    assert!(!accepts_nfa(&t, &chars("x")));
}

#[test]
fn complex_accessible_coaccessible_idempotent_on_literal() {
    let w = chars("abaaacc");
    let n = nfa_literal(&w);
    let a = n.accessible();
    let c = n.co_accessible();
    assert!(accepts_nfa(&a, &w));
    assert!(accepts_nfa(&c, &w));
}

#[test]
fn complex_literal_to_dfa_accepts_same_word() {
    let w = chars("abaaacc");
    let nfa = nfa_literal(&w);
    let dfa = nfa.to_dfa();
    assert!(accepts_dfa(&dfa, &w));
}

#[test]
fn complex_literal_to_dfa_rejects_reverse() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let dfa = nfa_literal(&w).to_dfa();
    assert!(!accepts_dfa(&dfa, &r));
}

#[test]
fn complex_union_is_symmetric_on_samples() {
    let w = chars("abaaacc");
    let r = reverse_word(&w);
    let lw = nfa_literal(&w);
    let lr = nfa_literal(&r);
    let a = lw.union(&lr);
    let b = lr.union(&lw);
    for sample in [&w, &r] {
        assert_eq!(accepts_nfa(&a, sample), accepts_nfa(&b, sample));
    }
    assert_eq!(
        accepts_nfa(&a, &chars("zzz")),
        accepts_nfa(&b, &chars("zzz"))
    );
}

#[test]
fn complex_star_star_accepts_same_samples_as_star() {
    let w = chars("ab");
    let forward = nfa_literal(&w);
    let revw = nfa_literal(&reverse_word(&w));
    let u = forward.union(&revw);
    let once = u.star();
    let twice = once.star();
    let samples = [chars(""), w.clone(), reverse_word(&w), chars("abab")];
    for s in &samples {
        assert_eq!(accepts_nfa(&once, s), accepts_nfa(&twice, s));
    }
}

#[test]
fn complex_reverse_concat_relates_to_concat_reverse() {
    let u = chars("ab");
    let v = chars("cd");
    let c_fwd = nfa_literal(&u).concatenate(&nfa_literal(&v));
    let ru = reverse_word(&u);
    let rv = reverse_word(&v);
    let c_rev_inner = nfa_literal(&rv).concatenate(&nfa_literal(&ru));
    let c = c_fwd.reverse();
    assert_eq!(
        accepts_nfa(&c, &chars("dcba")),
        accepts_nfa(&c_rev_inner, &chars("dcba"))
    );
}

#[test]
fn complex_minimize_via_to_dfa_preserves_abaaacc() {
    let w = chars("abaaacc");
    let dfa = nfa_literal(&w).to_minimized_dfa();
    assert!(accepts_dfa(&dfa, &w));
}

#[test]
fn complex_even_union_balanced_brackets_style_star() {
    let open = chars("(");
    let close = chars(")");
    let pair = nfa_literal(&open).concatenate(&nfa_literal(&close));
    let s = pair.star();
    assert!(accepts_nfa(&s, &[]));
    assert!(accepts_nfa(&s, &chars("()")));
    assert!(accepts_nfa(&s, &chars("()()")));
    let rev = s.reverse();
    assert!(accepts_nfa(&rev, &chars(")(")));
}

#[test]
fn try_new_singleton_words_transition_only_for_listed_symbols() {
    let nfa = SimpleNFA::try_new_singleton_words(['a', 'b'], ['a']).unwrap();
    let dfa = nfa.to_dfa();
    assert!(dfa.accepts(&['a']));
    assert!(!dfa.accepts(&['b']));
    assert_eq!(nfa.successors(0, &'b').collect::<Vec<_>>(), vec![]);
}

#[test]
fn try_new_singleton_words_errors_when_listed_symbol_missing_from_alphabet() {
    let err = SimpleNFA::try_new_singleton_words(['x'], ['x', 'y']).unwrap_err();
    assert_eq!(err, SimpleBuildError::SymbolNotInAlphabet('y'));
}

#[test]
fn try_new_singleton_words_empty_symbol_set_has_no_accepting_run_from_initial() {
    let dfa = SimpleNFA::try_new_singleton_words(['a', 'b'], [])
        .unwrap()
        .to_dfa();
    assert!(!dfa.accepts(&[]));
    assert!(!dfa.accepts(&['a']));
}
