# automata

Deterministic and nondeterministic finite-automata algorithms in Rust.

This crate focuses on finite automata operations with a stable, explicit
trait-layer API and no ε-transitions in the public trait surface (no regex
support).

## What it does

The library provides:
- Base trait layers for generic automata concepts (`general`)
- A finiteness layer for algorithms that require finite state sets and finite alphabets (`finite`)
- Concrete reference implementations (`simple`), plus a set of integration tests

The main emphasis is to keep the public trait layer free of ε-transitions.

## Implemented

High-level operations (typically provided as trait methods):
- Determinization (`to_dfa`) for NFAs
- Boolean/structural operations: `union`, `intersection`, `difference`, `concatenate`, `star`
- Closure operations: `reverse`, `trimmed`, `accessible`, `co_accessible`
- DFA completion and complement: `complete`, `complement` (requires a total DFA pipeline)
- Brzozowski minimization (`minimize`)

Debug/interop helpers:
- `FiniteAutomaton::to_dot` (Graphviz DOT) for visual inspection
- `DeterministicFiniteAutomaton::to_matrix` (DFA transition matrix)

## Quick example

```rust
use automata::simple::SimpleDFA;
use automata::finite::deterministic::DeterministicFiniteAutomaton;
use automata::general::deterministic::DeterministicAutomaton;

let alphabet = ['a'];
// 0 = even length, 1 = odd length
let edges = [(0usize, 'a', 1usize), (1usize, 'a', 0usize)];
let dfa = SimpleDFA::try_new(2, 0, [0], alphabet, edges).unwrap();

assert!(dfa.accepts(&[]));
assert!(!dfa.accepts(&['a']));
assert!(dfa.accepts(&['a', 'a']));
```

## Module organization

- [`general`](src/general/): the base `Automaton` trait + determinism/nondeterminism helpers
- [`finite`](src/finite/): finiteness-bound traits and algorithmic operations
- [`simple`](src/simple/): `SimpleDFA` / `SimpleNFA` concrete implementations

## License

MIT OR Apache-2.0

