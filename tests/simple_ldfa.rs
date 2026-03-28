use automata_core::{
    finite::NonDeterministicFiniteAutomaton,
    labeled::{
        arbitrary::{DeterministicLabeledAutomaton, LabeledAutomaton},
        finite::{DeterministicFiniteLabeledAutomaton, NonDeterministicFiniteLabeledAutomaton},
        simple::SimpleLabeledDFA,
    },
    simple::SimpleNFA,
};

////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////

fn nfa_singleton(words: &str) -> SimpleNFA {
    let alphabet = words.chars();
    let symbols = words.chars();
    SimpleNFA::try_new_singleton_words(alphabet, symbols).unwrap()
}

fn nfa_a_plus() -> SimpleNFA {
    let a = nfa_singleton("a");
    a.concatenate(&a.star())
}

fn nfa_letter() -> SimpleNFA {
    let letter = nfa_singleton("abcdefghijklmnopqrstuvwxyz");
    letter
}

fn nfa_ident() -> SimpleNFA {
    let letter = nfa_letter();
    letter.concatenate(&letter.star())
}

fn ldfa_ident_or_a_plus() -> SimpleLabeledDFA<u8> {
    let ident = nfa_ident().label_all_accepting_states_with(0u8);
    let a_plus = nfa_a_plus().label_all_accepting_states_with(1u8);
    let lnfa = ident.union(&a_plus);
    lnfa.to_dfa_by(Ord::min).minimize()
}

fn ldfa_a_plus_or_ident() -> SimpleLabeledDFA<u8> {
    let ident = nfa_ident().label_all_accepting_states_with(1u8);
    let a_plus = nfa_a_plus().label_all_accepting_states_with(0u8);
    let lnfa = ident.union(&a_plus);
    lnfa.to_dfa_by(Ord::min).minimize()
}

////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////

#[test]
fn ldfa_ident_or_a_plus_state_set_is_correct() {
    let ldfa = ldfa_ident_or_a_plus();
    let states_count = ldfa.states().count();
    assert_eq!(states_count, 2);
}

#[test]
fn ldfa_a_plus_or_ident_state_set_is_correct() {
    let ldfa = ldfa_a_plus_or_ident();
    let states_count = ldfa.states().count();
    assert_eq!(states_count, 3);
}

#[test]
fn ldfa_ident_or_a_plus_state_set_labels_are_correct() {
    let ldfa = ldfa_ident_or_a_plus();
    let label_of_aa = ldfa.get_label_of_word(&['a', 'a']);
    let label_of_bb = ldfa.get_label_of_word(&['b', 'b']);
    assert_eq!(label_of_aa, Some(0u8));
    assert_eq!(label_of_bb, Some(0u8));
}

#[test]
fn ldfa_a_plus_or_ident_state_set_labels_are_correct() {
    let ldfa = ldfa_a_plus_or_ident();
    let label_of_aa = ldfa.get_label_of_word(&['a', 'a']);
    let label_of_bb = ldfa.get_label_of_word(&['b', 'b']);
    assert_eq!(label_of_aa, Some(0u8));
    assert_eq!(label_of_bb, Some(1u8));
}

#[test]
fn ldfa_drop_labels() {
    let ldfa = ldfa_ident_or_a_plus();
    let ldfa_dropped_labels = ldfa.drop_labels();
    assert_eq!(ldfa_dropped_labels.get_label_of_word(&['a', 'b']), Some(()));
}
