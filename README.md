# Rust Calculator

A calculator with a shared expression evaluator, a CLI, and a native GUI, all in Rust.

- Supports `+ - * / % ^`, parentheses, unary minus, and decimals, with standard operator precedence (`^` is right-associative).
- The GUI follows the OS's light/dark theme live and is styled after macOS design conventions (see `docs/adr/` and `docs/research/`).

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (install via `rustup` or your OS package manager)

## Build

```sh
cargo build --release
```

## Run

**GUI** (a native window with a button grid):

```sh
cargo run --release --bin gui
```

**CLI** — REPL mode:

```sh
cargo run --release --bin calc
```

**CLI** — one-shot mode (evaluates the given expression and exits):

```sh
cargo run --release --bin calc -- "2 + 3 * 4"
```

## Test

```sh
cargo test
```

## Project layout

- `src/calculator.rs` — the shared tokenizer/parser/evaluator, used by both binaries
- `src/main.rs` — the `calc` CLI binary
- `src/bin/gui.rs` — the `gui` binary
- `assets/fonts/` — the embedded Inter font (SIL OFL 1.1, see `Inter-OFL.txt`)
- `docs/adr/` — architecture decision records
- `docs/research/` — research notes backing some of the above decisions
- `CONTEXT.md` — glossary of project-specific terms
