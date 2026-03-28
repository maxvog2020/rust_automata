// TODO: docs
mod automaton;
mod deterministic;
mod nondeterministic;

pub mod parsing;

pub use automaton::FiniteLabeledAutomaton;
pub use deterministic::DeterministicFiniteLabeledAutomaton;
pub use nondeterministic::NonDeterministicFiniteLabeledAutomaton;
