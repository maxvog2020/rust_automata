use automata_core::finite::NonDeterministicFiniteAutomaton;
use automata_core::finite::parsing::{parse_by_longest_match, ParseResult};
use automata_core::arbitrary::Automaton;
use automata_core::simple::{SimpleDFA, SimpleNFA};

////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////

fn parse_consumed(word: &[char], dfa: &SimpleDFA) -> Vec<ParseResult<usize>> {
    parse_by_longest_match(dfa, word).unwrap()
}

fn word_bounds(parse: &[ParseResult<usize>]) -> Vec<(usize, usize)> {
    parse
        .iter()
        .map(|pr| (pr.position_in_word, pr.size))
        .collect()
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

fn dfa_ab_chain() -> SimpleDFA {
    let a = nfa_singleton("a");
    let b = nfa_singleton("b");
    let nfa = a.concatenate(&b);
    nfa.to_minimized_dfa()
}

////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////

#[test]
fn parse_empty_word_yields_no_tokens() {
    let dfa = dfa_ab_chain();
    assert_eq!(parse_by_longest_match(&dfa, &[]).unwrap(), vec![]);
}

#[test]
fn single_symbol_one_token() {
    let dfa = SimpleDFA::try_new(2, 0, [1], ['x'], [(0, 'x', 1)]).unwrap();
    let got = parse_by_longest_match(&dfa, &['x']).unwrap();
    assert_eq!(
        got,
        vec![ParseResult {
            state: 1,
            position_in_word: 0,
            size: 1,
        }]
    );
}

#[test]
fn two_letter_word_one_token() {
    let dfa = dfa_ab_chain();
    let accepting = dfa.accepting_states().collect::<Vec<_>>();
    let word = ['a', 'b'];
    let got = parse_by_longest_match(&dfa, &word).unwrap();
    assert_eq!(accepting.len(), 1);
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ParseResult {
            state: accepting[0],
            position_in_word: 0,
            size: 2,
        }
    );
}

#[test]
fn fails_when_first_symbol_has_no_transition() {
    let dfa = dfa_ab_chain();
    assert!(parse_by_longest_match(&dfa, &['z']).is_none());
}

#[test]
fn fails_when_first_token_cannot_end_accepting() {
    // 0-a->1 (1 not accepting): no accept-terminated prefix from initial.
    let dfa = SimpleDFA::try_new(2, 0, [], ['a'], [(0, 'a', 1)]).unwrap();
    assert!(parse_by_longest_match(&dfa, &['a']).is_none());
}

#[test]
fn fails_when_no_pattern_matches_remainder() {
    // "aa" but only 0-a->1-b->2 (need 'b' after first 'a'); second 'a' cannot start a token.
    let dfa = dfa_ab_chain();
    assert!(parse_by_longest_match(&dfa, &['a', 'a']).is_none());
}

#[test]
fn self_loop_accepting_consumes_whole_input_as_one_token() {
    let dfa = SimpleDFA::try_new(1, 0, [0], ['a'], [(0, 'a', 0)]).unwrap();
    let word = ['a', 'a', 'a', 'a'];
    let got = parse_by_longest_match(&dfa, &word).unwrap();
    assert_eq!(
        got,
        vec![ParseResult {
            state: 0,
            position_in_word: 0,
            size: 4,
        }]
    );
}

/// Same single-char edge twice: each token is one `a`, always from state 0.
#[test]
fn two_tokens_repeated_single_char_pattern() {
    let dfa = SimpleDFA::try_new(2, 0, [1], ['a'], [(0, 'a', 1)]).unwrap();
    let got = parse_by_longest_match(&dfa, &['a', 'a']).unwrap();
    assert_eq!(got.len(), 2);
    assert_eq!(
        got,
        vec![
            ParseResult {
                state: 1,
                position_in_word: 0,
                size: 1,
            },
            ParseResult {
                state: 1,
                position_in_word: 1,
                size: 1,
            },
        ]
    );
}

/// `ab` then `cd`, each recognized from state 0; state 2 has no `c` edge.
fn dfa_two_tokens_ab_then_cd() -> SimpleDFA {
    SimpleDFA::try_new(
        5,
        0,
        [2, 4],
        ['a', 'b', 'c', 'd'],
        [
            (0, 'a', 1),
            (1, 'b', 2),
            (0, 'c', 3),
            (3, 'd', 4),
        ],
    )
    .unwrap()
}

#[test]
fn two_tokens_ab_then_cd() {
    let dfa = dfa_two_tokens_ab_then_cd();
    let word = ['a', 'b', 'c', 'd'];
    let got = parse_by_longest_match(&dfa, &word).unwrap();
    assert_eq!(
        got,
        vec![
            ParseResult {
                state: 2,
                position_in_word: 0,
                size: 2,
            },
            ParseResult {
                state: 4,
                position_in_word: 2,
                size: 2,
            },
        ]
    );
}

#[test]
fn three_single_char_tokens() {
    let dfa = SimpleDFA::try_new(
        4,
        0,
        [1, 2, 3],
        ['a', 'b', 'c'],
        [(0, 'a', 1), (0, 'b', 2), (0, 'c', 3)],
    )
    .unwrap();
    let got = parse_by_longest_match(&dfa, &['a', 'b', 'c']).unwrap();
    assert_eq!(
        got,
        vec![
            ParseResult {
                state: 1,
                position_in_word: 0,
                size: 1,
            },
            ParseResult {
                state: 2,
                position_in_word: 1,
                size: 1,
            },
            ParseResult {
                state: 3,
                position_in_word: 2,
                size: 1,
            },
        ]
    );
}

#[test]
fn longest_munch_first_token_then_second() {
    // Longest accept-terminated prefix of "aab" from 0 is "aa"; then `b` from 0.
    let dfa = SimpleDFA::try_new(
        4,
        0,
        [2, 3],
        ['a', 'b'],
        [(0, 'a', 1), (1, 'a', 2), (0, 'b', 3)],
    )
    .unwrap();
    let got = parse_by_longest_match(&dfa, &['a', 'a', 'b']).unwrap();
    assert_eq!(
        got,
        vec![
            ParseResult {
                state: 2,
                position_in_word: 0,
                size: 2,
            },
            ParseResult {
                state: 3,
                position_in_word: 2,
                size: 1,
            },
        ]
    );
}

#[test]
fn empty_word_ok_for_multi_pattern_dfa() {
    let dfa = dfa_two_tokens_ab_then_cd();
    assert_eq!(parse_by_longest_match(&dfa, &[]).unwrap(), vec![]);
}

#[test]
fn fails_when_second_token_incomplete() {
    let dfa = dfa_two_tokens_ab_then_cd();
    assert!(parse_by_longest_match(&dfa, &['a', 'b', 'c']).is_none());
}

#[test]
fn four_tokens_mixed_lengths() {
    let dfa = SimpleDFA::try_new(
        9,
        0,
        [2, 5, 8],
        ['p', 'q', 'x', 'y', 'z'],
        [
            (0, 'x', 1),
            (1, 'y', 2),
            (0, 'z', 3),
            (3, 'z', 4),
            (4, 'z', 5),
            (0, 'p', 6),
            (0, 'q', 7),
            (7, 'q', 8),
        ],
    )
    .unwrap();
    let word: Vec<char> = "xyzzzqq".chars().collect();
    let got = parse_by_longest_match(&dfa, &word).unwrap();
    assert_eq!(
        got,
        vec![
            ParseResult {
                state: 2,
                position_in_word: 0,
                size: 2,
            },
            ParseResult {
                state: 5,
                position_in_word: 2,
                size: 3,
            },
            ParseResult {
                state: 8,
                position_in_word: 5,
                size: 2,
            },
        ]
    );
}

#[test]
fn parse_singleton_words_each_listed_symbol_is_one_segment() {
    let dfa = SimpleNFA::try_new_singleton_words(['a', 'b', '+'], ['a', '+']).unwrap().to_dfa();
    let got = parse_by_longest_match(&dfa, &['a', '+', 'a']).unwrap();
    assert_eq!(
        got,
        vec![
            ParseResult {
                state: 1,
                position_in_word: 0,
                size: 1,
            },
            ParseResult {
                state: 1,
                position_in_word: 1,
                size: 1,
            },
            ParseResult {
                state: 1,
                position_in_word: 2,
                size: 1,
            },
        ]
    );
}

#[test]
fn parse_singleton_words_full_alphabet_each_char_one_segment() {
    let dfa = SimpleNFA::try_new_singleton_words(['x', 'y', 'z'], ['x', 'y', 'z']).unwrap().to_dfa();
    let word = ['x', 'y', 'z', 'x'];
    let got = parse_by_longest_match(&dfa, &word).unwrap();
    assert_eq!(got.len(), 4);
    for (i, pr) in got.iter().enumerate() {
        assert_eq!(pr.state, 1);
        assert_eq!(pr.position_in_word, i);
        assert_eq!(pr.size, 1);
    }
}

#[test]
fn parse_singleton_words_fails_when_first_char_not_singleton() {
    let dfa = SimpleNFA::try_new_singleton_words(['a', 'b'], ['a']).unwrap().to_dfa();
    assert!(parse_by_longest_match(&dfa, &['b']).is_none());
    assert!(parse_by_longest_match(&dfa, &['b', 'a']).is_none());
}

#[test]
fn parse_singleton_words_fails_after_good_prefix_when_next_char_not_singleton() {
    let dfa = SimpleNFA::try_new_singleton_words(['a', 'b'], ['a']).unwrap().to_dfa();
    assert!(parse_by_longest_match(&dfa, &['a', 'b']).is_none());
}

#[test]
fn parse_singleton_words_empty_input_yields_empty_parse() {
    let dfa = SimpleNFA::try_new_singleton_words(['a', 'b', '+'], ['a', '+']).unwrap().to_dfa();
    assert_eq!(parse_by_longest_match(&dfa, &[]).unwrap(), vec![]);
}

#[test]
fn parse_singleton_words_digit_like_symbols_stream() {
    let dfa = SimpleNFA::try_new_singleton_words(
        ['0', '1', '2', '+', '-', ' ', 'x'],
        ['0', '1', '2', '+', '-'],
    ).unwrap().to_dfa();
    let word: Vec<char> = "0+1-2".chars().collect();
    let got = parse_consumed(&word, &dfa);
    assert_eq!(got.len(), 5);
    assert_eq!(
        word_bounds(&got),
        vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)]
    );
}

#[test]
fn parse_singleton_words_with_empty_constructor_symbol_set_only_empty_input() {
    let dfa = SimpleNFA::try_new_singleton_words(['a', 'b'], []).unwrap().to_dfa();
    assert_eq!(parse_by_longest_match(&dfa, &[]).unwrap(), vec![]);
    assert!(parse_by_longest_match(&dfa, &['a']).is_none());
}

#[test]
fn expression_stream_one_fragment_token_boundaries() {
    let dfa = dfa_number_operator_space();
    let word: Vec<char> = "42 + ( 3 * 17 ) - 9 / 2 ".chars().collect();
    let got = parse_by_longest_match(&dfa, &word).unwrap();
    let expected_bounds: Vec<(usize, usize)> = vec![
        (0, 2),
        (2, 1),
        (3, 1),
        (4, 1),
        (5, 1),
        (6, 1),
        (7, 1),
        (8, 1),
        (9, 1),
        (10, 1),
        (11, 2),
        (13, 1),
        (14, 1),
        (15, 1),
        (16, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (20, 1),
        (21, 1),
        (22, 1),
        (23, 1),
    ];
    assert_eq!(word_bounds(&got), expected_bounds);
    let sum: usize = got.iter().map(|p| p.size).sum();
    assert_eq!(sum, word.len());
}

#[test]
fn expression_stream_1200_chars_full_cover() {
    let dfa = dfa_number_operator_space();
    let word = word_expression_stream("42 + ( 3 * 17 ) - 9 / 2 ", 1200);
    let got = parse_by_longest_match(&dfa, &word).expect("parse");
    let sum: usize = got.iter().map(|p| p.size).sum();
    assert_eq!(sum, word.len());
    assert_eq!(got.first().unwrap().position_in_word, 0);
    assert_eq!(
        got.last().unwrap().position_in_word + got.last().unwrap().size,
        word.len()
    );
}

#[test]
fn ba_star_built_minimizes_to_single_token_on_long_word() {
    let dfa = dfa_ba_star();
    let word: Vec<char> = std::iter::once('b').chain(std::iter::repeat_n('a', 1199)).collect();
    assert_eq!(word.len(), 1200);
    let got = parse_by_longest_match(&dfa, &word).unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].position_in_word, 0);
    assert_eq!(got[0].size, 1200);
}

#[test]
fn alternating_ca_ba_blocks_parse_covers_1180_chars() {
    let dfa = dfa_ca_plus_or_ba_plus();
    let target_len = 1180;
    let mut word = Vec::with_capacity(target_len);
    let blocks: &[&[char]] = &[&['c', 'a', 'a', 'a'], &['b', 'a', 'a', 'a']];
    let mut i = 0;
    while word.len() < target_len {
        for &ch in blocks[i % 2] {
            if word.len() >= target_len {
                break;
            }
            word.push(ch);
        }
        i += 1;
    }
    let got = parse_by_longest_match(&dfa, &word).expect("parse");
    let sum: usize = got.iter().map(|p| p.size).sum();
    assert_eq!(sum, word.len());
}
