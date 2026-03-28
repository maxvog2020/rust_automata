use std::hint::black_box;
use std::iter::repeat_n;

use automata_core::finite::NonDeterministicFiniteAutomaton;
use automata_core::finite::parsing::parse_by_longest_match;
use automata_core::simple::{SimpleDFA, SimpleNFA};
use criterion::{Criterion, criterion_group, criterion_main};
use automata_core::labeled::finite::NonDeterministicFiniteLabeledAutomaton;

const LONG: usize = 1200;
const LONG_ALT: usize = 1180;

////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////

fn word_ba_star(len: usize) -> Vec<char> {
    assert!(len >= 1);
    std::iter::once('b').chain(repeat_n('a', len - 1)).collect()
}

fn word_repeated_ba_plus(target_len: usize) -> Vec<char> {
    let chunk: Vec<char> = std::iter::once('b').chain(repeat_n('a', 4)).collect();
    chunk.iter().copied().cycle().take(target_len).collect()
}

fn word_alternating_ca_ba(target_len: usize) -> Vec<char> {
    let mut v = Vec::with_capacity(target_len);
    let blocks: &[&[char]] = &[&['c', 'a', 'a', 'a'], &['b', 'a', 'a', 'a']];
    let mut i = 0;
    while v.len() < target_len {
        for &ch in blocks[i % 2] {
            if v.len() >= target_len {
                break;
            }
            v.push(ch);
        }
        i += 1;
    }
    v
}

fn word_expression_stream(fragment: &str, target_len: usize) -> Vec<char> {
    let mut v = Vec::with_capacity(target_len);
    while v.len() < target_len {
        let take = (target_len - v.len()).min(fragment.len());
        v.extend(fragment.chars().take(take));
    }
    v
}

fn nfa_singleton(words: &str) -> SimpleNFA {
    let alphabet = words.chars();
    let symbols = words.chars();
    SimpleNFA::try_new_singleton_words(alphabet, symbols).unwrap()
}

fn nfa_digit() -> SimpleNFA {
    nfa_singleton("0123456789")
}

fn nfa_number() -> SimpleNFA {
    nfa_digit().concatenate(&nfa_digit().star())
}

fn nfa_space() -> SimpleNFA {
    nfa_singleton(" ")
}

fn nfa_operator() -> SimpleNFA {
    nfa_singleton("+-*/()")
}

fn dfa_number_operator_space() -> SimpleDFA {
    let number = nfa_number();
    let operator = nfa_operator();
    let space = nfa_space();
    let nfa = SimpleNFA::union_all(&[number, operator, space]).unwrap();
    nfa.to_minimized_dfa()
}

fn dfa_ba_star() -> SimpleDFA {
    let a = nfa_singleton("a");
    let a_star = a.star();
    let b = nfa_singleton("b");
    let nfa = b.concatenate(&a_star);
    nfa.to_minimized_dfa()
}

fn dfa_ca_plus_or_ba_plus() -> SimpleDFA {
    let a = nfa_singleton("a");
    let a_plus = a.concatenate(&a.star());
    let ca_plus = nfa_singleton("c").concatenate(&a_plus.clone());
    let ba_plus = nfa_singleton("b").concatenate(&a_plus);
    ca_plus.union(&ba_plus).to_minimized_dfa()
}

////////////////////////////////////////////////////////////
// Benchmarks
////////////////////////////////////////////////////////////

fn bench_parse_ba_star(c: &mut Criterion) {
    let dfa = dfa_ba_star();
    let word = word_ba_star(LONG);
    c.bench_function("parse ba* DFA (built+min), 1200 chars one token", |b| {
        b.iter(|| {
            let r = parse_by_longest_match(black_box(&dfa), black_box(word.as_slice()));
            black_box(r)
        });
    });
}

fn bench_parse_ca_or_ba_plus(c: &mut Criterion) {
    let dfa = dfa_ca_plus_or_ba_plus();
    let word = word_repeated_ba_plus(LONG_ALT);
    c.bench_function(
        "parse (ca+)|(ba+) DFA (built+min), ~1180 chars ba+ tokens",
        |b| {
            b.iter(|| {
                let r = parse_by_longest_match(black_box(&dfa), black_box(word.as_slice()));
                black_box(r)
            });
        },
    );

    let word2 = word_alternating_ca_ba(LONG_ALT);
    c.bench_function(
        "parse (ca+)|(ba+) DFA (built+min), ~1180 chars c/b alt",
        |b| {
            b.iter(|| {
                let r = parse_by_longest_match(black_box(&dfa), black_box(word2.as_slice()));
                black_box(r)
            });
        },
    );
}

fn bench_parse_nat_ops_space(c: &mut Criterion) {
    let dfa = dfa_number_operator_space();
    let word = word_expression_stream("42 + ( 3 * 17 ) - 9 / 2 ", LONG);
    c.bench_function("parse lexer DFA (built+min), 1200 chars expr stream", |b| {
        b.iter(|| {
            let r = parse_by_longest_match(black_box(&dfa), black_box(word.as_slice()));
            black_box(r)
        });
    });
}

fn bench_parse_singleton_words_dfa(c: &mut Criterion) {
    let alphabet: Vec<char> = ('a'..='z').chain('0'..='9').collect();
    let dfa =
        SimpleNFA::try_new_singleton_words(alphabet.iter().copied(), alphabet.iter().copied())
            .unwrap()
            .to_minimized_dfa();
    let word: Vec<char> = std::iter::repeat("a7k2m9p1")
        .flat_map(|s| s.chars())
        .take(LONG)
        .collect();
    c.bench_function("parse singleton-words DFA (min), 1200 chars", |b| {
        b.iter(|| {
            let r = parse_by_longest_match(black_box(&dfa), black_box(word.as_slice()));
            black_box(r)
        });
    });
}

fn word_digit_heavy_expression(target_len: usize) -> Vec<char> {
    let fragment: Vec<char> = "12345678901234567890 + 9876543210 * ( 42 ) "
        .chars()
        .collect();
    let mut v = Vec::with_capacity(target_len);
    while v.len() < target_len {
        let take = (target_len - v.len()).min(fragment.len());
        v.extend(fragment.iter().take(take).copied());
    }
    v
}

fn bench_parse_digit_heavy_expr(c: &mut Criterion) {
    let dfa = dfa_number_operator_space();
    let word = word_digit_heavy_expression(LONG);
    c.bench_function("parse lexer DFA (built+min), 1200 chars digit-heavy", |b| {
        b.iter(|| {
            let r = parse_by_longest_match(black_box(&dfa), black_box(word.as_slice()));
            black_box(r)
        });
    });
}

criterion_group!(
    benches,
    bench_parse_ba_star,
    bench_parse_ca_or_ba_plus,
    bench_parse_nat_ops_space,
    bench_parse_singleton_words_dfa,
    bench_parse_digit_heavy_expr,
);
criterion_main!(benches);
