# automata_core

[![Crates.io](https://img.shields.io/crates/v/automata_core.svg)](https://crates.io/crates/automata_core)
[![Docs.rs](https://docs.rs/automata_core/badge.svg)](https://docs.rs/automata_core)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

Deterministic and nondeterministic automaton algorithms in Rust.

The crate is layered:

- **`labeled`** — automata whose states may carry an output **label** (`LabeledAutomaton`, DFA/NFA labeled traits, `SimpleLabeledDFA` / `SimpleLabeledNFA`).
- **`labeled::arbitrary`** — labeled traits **without** assuming a finite state set or alphabet (iterators over states and symbols are not required to end).
- **`arbitrary`** — same traits with `Label = ()` (unlabeled façade: accepting states are those with `Some(())`).
- **`finite`** / **`labeled::finite`** — finiteness bounds so algorithms can enumerate states and symbols (subset construction, closure operations, completion, minimization, longest-match parsing, etc.).

The public trait layer does not include ε-transitions.

This crate is not affiliated with the unrelated crate on crates.io named
`automata` (https://crates.io/crates/automata).

## What it does

The library provides:

- Trait layers for labeled and unlabeled automata (`labeled`, `arbitrary`, `finite`)
- Concrete reference types in `simple` (`SimpleDFA` / `SimpleNFA` are aliases with `Label = ()` over `labeled::simple`)

High-level operations (typically trait methods on the **finite** NFA/DFA traits):

- Determinization (`to_dfa` / `to_dfa_by`) for NFAs
- Boolean/structural operations: `union`, `intersection`, `difference`, `concatenate`, `star`
- N-ary helpers: `union_all`, `intersect_all`, `concatenate_all`
- Closure-style operations: `reverse`, `trimmed`, `accessible`, `co_accessible`
- DFA completion and complement: `complete`, `complement`
- DFA minimization (`minimize`; the `SimpleLabeledDFA` implementation uses Hopcroft’s algorithm)
- Lexer-style longest-match parsing for DFAs (`parse_by_longest_match`)

## Quick example

```rust
use automata_core::simple::SimpleDFA;
use automata_core::arbitrary::DeterministicAutomaton;
use automata_core::finite::DeterministicFiniteAutomaton;

let alphabet = ['a'];
// 0 = even length, 1 = odd length
let edges = [(0usize, 'a', 1usize), (1usize, 'a', 0usize)];
let dfa = SimpleDFA::try_new(2, 0, [0], alphabet, edges).unwrap();

assert!(dfa.accepts(&[]));
assert!(!dfa.accepts(&['a']));
assert!(dfa.accepts(&['a', 'a']));
```

## Module organization

- [`src/labeled/`](src/labeled/) — labeled traits (`arbitrary`, `finite`, `simple`)
- [`src/arbitrary/`](src/arbitrary/) — unlabeled (`Label = ()`) re-export of the same trait shapes
- [`src/finite/`](src/finite/) — finite unlabeled automata and algorithms
- [`src/simple/`](src/simple/) — `SimpleDFA` / `SimpleNFA` aliases over `labeled::simple`

## WIP / status

This library is work-in-progress. While the core constructions and trait
APIs are in place, test coverage is still incomplete and some debug helpers
are not finished yet.

## License

MIT OR Apache-2.0
