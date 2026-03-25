use crate::general::Automaton;

/// Finite automata are automata whose state set and input alphabet can be
/// enumerated.
///
/// This trait is used as a *finiteness bound*: algorithms that need to loop
/// over all symbols or all states should require `FiniteAutomaton`.
pub trait FiniteAutomaton: Automaton {
    /// Render a Graphviz DOT graph.
    ///
    /// TODO: implement Graphviz DOT output including transition edges.
    ///
    /// The trait does not expose transition structure, so a complete DOT
    /// rendering requires additional information from concrete types.
    ///
    /// This default currently panics to make the missing feature explicit.
    fn to_dot(&self) -> String {
        todo!("Implement conversion to DOT format")
    }
}
