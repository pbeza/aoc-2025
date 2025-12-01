# Advent of Code 2025

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) in Rust.

## Structure

This is a Cargo workspace with:
- Shared library (`src/`) for common utilities
- Separate crate per day (`day01/`, `day02/`, etc.)

## Usage

Run a specific day:
```bash
cargo run -p day01
cargo run -p day02
```

Run with release optimizations:
```bash
cargo run -p day01 --release
```

Run tests:
```bash
cargo test -p day01
```

Build everything:
```bash
cargo build --workspace
```
