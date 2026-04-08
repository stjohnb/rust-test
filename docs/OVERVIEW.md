# rust-test: Overview

## Purpose

A personal Rust learning repository that works through exercises from the [Rust by Example](https://doc.rust-lang.org/rust-by-example/) book. It also contains a small standalone binary crate (`src/main.rs`) used for quick experimentation. The repo serves as a scratchpad for learning and practicing Rust concepts.

## Repository Structure

```
rust-test/
├── Cargo.toml          # Package: "guessing_game", edition 2018, depends on rand 0.3.14
├── src/
│   └── main.rs         # Standalone scratch binary (currently tests array indexing/panics)
└── rust-by-example/    # Exercises organized by chapter number
    ├── 1/              # Hello World, formatting, display, printing
    ├── 2/              # Primitives: arrays, tuples, literals
    ├── 3/              # Custom types: structs, enums, LinkedList
    ├── 4/              # Variable bindings: shadowing
    ├── 5/              # Types: casting, aliasing, inference, literals
    ├── 6/              # Conversions: From, Into, TryFrom
    ├── 8/              # Flow control: for, if-let, pointers, structs
    ├── 9/              # Functions/closures: capturing, input params, iterators, methods
    ├── 10/             # Modules: visibility, nested modules, file splitting
    ├── 11/             # Crates: library (rary.rs) and executable (executable.rs)
    ├── 12/             # Cargo: two mini-projects (foo/, bar/) with their own Cargo.toml
    ├── 13/             # Attributes: cfg, custom attributes
    ├── 15/             # Scoping: lifetimes, borrowing, bounds, refs, structs
    ├── 16/             # Traits: Clone, Derive, Drop, Iterator, impl blocks
    ├── 17/             # Macros: macro_rules!, designators
    └── 18/             # Error handling: Option, Map, Panic
```

Chapters 7 and 14 from Rust by Example are not yet present.

## Key Patterns

- **Chapter-per-directory**: Each `rust-by-example/<N>/` directory maps to one chapter of the Rust by Example book. Files within a chapter are standalone `.rs` source files illustrating specific concepts.
- **No shared build system for examples**: The `rust-by-example/` files are reference/study material — they are not wired into the root `Cargo.toml`. They are meant to be read and run individually (e.g., `rustc file.rs`), not as part of a workspace.
- **Two exceptions with their own Cargo.toml**: Chapter 11 (`rary.rs`/`executable.rs`) demonstrates crate linking, and Chapter 12 (`foo/`, `bar/`) are standalone Cargo projects demonstrating the Cargo build tool.
- **Main crate is a scratch binary**: `src/main.rs` is the actual compiled target of the root `Cargo.toml`. Its contents change frequently as it is used to test ad-hoc snippets (currently demonstrates out-of-bounds array indexing).

## Configuration

- **Edition**: Rust 2018 (`Cargo.toml`)
- **Dependency**: `rand = "0.3.14"` — included from the original guessing game tutorial, may be unused in the current `main.rs`
- No environment variables or external configuration required.
