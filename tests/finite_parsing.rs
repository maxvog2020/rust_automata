use automata_core::finite::parsing::{parse_by_longest_match, ParseResult};
use automata_core::simple::SimpleDFA;

fn literal_ab_chain() -> SimpleDFA {
    // 0 --a--> 1 --b--> 2 (2 is accepting)
    SimpleDFA::try_new(3, 0, [2], ['a', 'b'], [(0, 'a', 1), (1, 'b', 2)]).unwrap()
}

#[test]
fn parse_empty_word_yields_no_tokens() {
    let dfa = literal_ab_chain();
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
    let dfa = literal_ab_chain();
    let word = ['a', 'b'];
    let got = parse_by_longest_match(&dfa, &word).unwrap();
    assert_eq!(got.len(), 1);
    assert_eq!(
        got[0],
        ParseResult {
            state: 2,
            position_in_word: 0,
            size: 2,
        }
    );
}

#[test]
fn fails_when_first_symbol_has_no_transition() {
    let dfa = literal_ab_chain();
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
    let dfa = literal_ab_chain();
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
