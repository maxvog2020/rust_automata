// TODO: docs

mod automaton;
mod deterministic;
mod nondeterministic;

pub use automaton::LabeledAutomaton;
pub use deterministic::DeterministicLabeledAutomaton;
pub use nondeterministic::NonDeterministicLabeledAutomaton;
