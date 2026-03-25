use crate::general::automaton::Automaton;

// states and alphabet are finite
pub trait FiniteAutomaton: Automaton {
    fn to_dot(&self) -> String {
        todo!("Implement conversion to DOT format")
    }
}
