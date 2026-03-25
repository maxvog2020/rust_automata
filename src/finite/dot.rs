use crate::finite::automaton::FiniteAutomaton;

pub trait ToDot: FiniteAutomaton {
    fn to_dot(&self) -> String {
        todo!("ToDot::to_dot")
    }
}
