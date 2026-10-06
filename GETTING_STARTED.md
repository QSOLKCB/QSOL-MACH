# Getting Started

## Requirements

- Rust 1.85 or newer
- Cargo

The repository contains no third-party Rust dependencies in Phase 0.

## Build

    cargo build

## Run

    cargo run --bin qsol-mach

## Test

    cargo test --all-targets

## Full local CI equivalent

    cargo fmt --check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test --all-targets

## Current scope

Phase 0 accepts already-decoded typed actions. It does not yet parse JSON, call an LLM, speak MCP, perform network I/O, or sandbox external processes. Those are deliberately kept outside the kernel until the runtime semantics are pinned by tests.
