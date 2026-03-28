use core::fmt;

/// Error returned when building a `SimpleDFA` or `SimpleNFA`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleBuildError {
    /// The initial state must satisfy `initial < state_count`.
    InitialOutOfRange { initial: usize, state_count: usize },
    /// A state in `accepting` must satisfy `state < state_count`.
    StateOutOfRange { state: usize, state_count: usize },
    /// A transition source must satisfy `from < state_count`.
    TransitionFromOutOfRange { from: usize, state_count: usize },
    /// A transition target must satisfy `to < state_count`.
    TransitionToOutOfRange { to: usize, state_count: usize },
    /// Transition symbol must be part of the declared alphabet.
    SymbolNotInAlphabet(char),
    /// Deterministic automata must not have duplicate transitions for
    /// `(state, symbol)`.
    DuplicateDeterministicTransition { state: usize, symbol: char },
}

impl fmt::Display for SimpleBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimpleBuildError::InitialOutOfRange {
                initial,
                state_count,
            } => write!(
                f,
                "initial state {initial} is out of range for state_count {state_count}"
            ),
            SimpleBuildError::StateOutOfRange { state, state_count } => write!(
                f,
                "state {state} is out of range for state_count {state_count}"
            ),
            SimpleBuildError::TransitionFromOutOfRange { from, state_count } => write!(
                f,
                "transition source {from} is out of range for state_count {state_count}"
            ),
            SimpleBuildError::TransitionToOutOfRange { to, state_count } => write!(
                f,
                "transition target {to} is out of range for state_count {state_count}"
            ),
            SimpleBuildError::SymbolNotInAlphabet(a) => {
                write!(f, "symbol {a:?} is not in the alphabet")
            }
            SimpleBuildError::DuplicateDeterministicTransition { state, symbol } => write!(
                f,
                "duplicate transition ({state}, {symbol:?}) for a deterministic automaton"
            ),
        }
    }
}

impl std::error::Error for SimpleBuildError {}
