//! Lexer-style **longest-match** parsing for deterministic finite automata.
//!
//! # Semantics
//!
//! [`parse_by_longest_match`] walks the input from left to right. At each
//! step it assumes the automaton is again in
//! [`DeterministicLabeledAutomaton::initial_state`](crate::labeled::arbitrary::DeterministicLabeledAutomaton::initial_state)
//! (as after a prior token in a typical hand-written lexer). From that state it finds the
//! **longest** prefix of the unread suffix that can be read with one
//! [`transition`](crate::labeled::arbitrary::DeterministicLabeledAutomaton::transition) per
//! symbol such that the **state after the last symbol of that prefix carries a
//! label** ([`LabeledAutomaton::get_label`](crate::labeled::arbitrary::LabeledAutomaton::get_label) is [`Some`]).
//!
//! That prefix is one **token**. The procedure repeats until the whole `word`
//! is consumed. If at some point no positive-length accepting prefix exists, or
//! a step along the chosen prefix is undefined, parsing returns [`None`].
//! The empty word yields [`Some`] with an empty vector.
//!
//! Each [`ParseResult`] describes one token: [`ParseResult::position_in_word`]
//! is where it starts in `word`, [`ParseResult::size`] is its length, and
//! [`ParseResult::state`] is the automaton state **after** reading the token
//! (a labeled / final state when the implementation is consistent).

use std::collections::HashMap;
use std::hash::Hash;

use crate::labeled::finite::DeterministicFiniteLabeledAutomaton;

/// One token emitted by [`parse_by_longest_match`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseResult<State> {
    /// State **after** consuming this token.
    pub state: State,
    /// Start index of this token in the input word (inclusive).
    pub position_in_word: usize,
    /// Number of input symbols in this token.
    pub size: usize,
}

/// One token emitted by [`parse_by_longest_match_labeled`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LabeledParseResult<State, Label> {
    /// State **after** consuming this token.
    pub state: State,
    /// Start index of this token in the input word (inclusive).
    pub position_in_word: usize,
    /// Number of input symbols in this token.
    pub size: usize,
    /// Label of the state **after** consuming this token.
    pub label: Label,
}

/// Split `word` into tokens using longest-match, **from [`initial_state`] each
/// time** after a token is consumed.
///
/// [`initial_state`]: crate::labeled::arbitrary::DeterministicLabeledAutomaton::initial_state
pub fn parse_by_longest_match<
    Label: Hash + Eq + Clone,
    A: DeterministicFiniteLabeledAutomaton<Label>,
>(
    automaton: &A,
    word: &[A::Input],
) -> Option<Vec<ParseResult<A::State>>> {
    let word_length = word.len();
    if word_length == 0 {
        return Some(Vec::new());
    }

    let dp = longest_accept_prefix_lengths(automaton, word);
    let initial = automaton.initial_state();

    let mut result = Vec::new();
    let mut current_position = 0;

    while current_position < word_length {
        let token_length = dp.get(&initial).unwrap()[current_position];

        if token_length == 0 {
            return None;
        }

        let segment_start = current_position;
        let segment = &word[current_position..current_position + token_length];
        let last_state = automaton.run_from(initial, segment)?;
        current_position += token_length;

        result.push(ParseResult {
            state: last_state,
            position_in_word: segment_start,
            size: token_length,
        });
    }

    Some(result)
}

/// Split `word` into tokens using longest-match, **from [`initial_state`] each
/// time** after a token is consumed, and return the labels of the tokens.
///
/// [`initial_state`]: crate::labeled::arbitrary::DeterministicLabeledAutomaton::initial_state
pub fn parse_by_longest_match_labeled<
    Label: Hash + Eq + Clone,
    A: DeterministicFiniteLabeledAutomaton<Label>,
>(
    automaton: &A,
    word: &[A::Input],
) -> Option<Vec<LabeledParseResult<A::State, Label>>> {
    let tokens = parse_by_longest_match(automaton, word)?;
    let result = tokens
        .into_iter()
        .map(|token| {
            let label = automaton.get_label(token.state).unwrap();
            enrich_with_label(token, label)
        })
        .collect();
    Some(result)
}

fn enrich_with_label<State, Label>(
    result: ParseResult<State>,
    label: Label,
) -> LabeledParseResult<State, Label> {
    LabeledParseResult {
        state: result.state,
        position_in_word: result.position_in_word,
        size: result.size,
        label: label,
    }
}

fn longest_accept_prefix_lengths<
    Label: Hash + Eq + Clone,
    A: DeterministicFiniteLabeledAutomaton<Label>,
>(
    automaton: &A,
    word: &[A::Input],
) -> HashMap<A::State, Vec<usize>> {
    let word_length = word.len();
    let states = automaton.states_set();

    let mut dp: HashMap<A::State, Vec<usize>> = HashMap::new();
    for &state in &states {
        dp.insert(state, vec![0; word_length + 1]);
    }

    for i in (0..word_length).rev() {
        for &state in &states {
            let input = &word[i];
            let Some(next_state) = automaton.transition(state, input) else {
                continue;
            };

            let mut max_len = 0;
            if automaton.has_label(next_state) {
                max_len = 1;
            }

            let next_value = dp.get(&next_state).unwrap()[i + 1];
            if next_value > 0 {
                max_len = 1 + next_value;
            }

            dp.get_mut(&state).unwrap()[i] = max_len;
        }
    }

    dp
}
