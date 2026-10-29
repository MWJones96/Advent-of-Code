# Advent of Code in Rust

Solutions to the [Advent of Code](https://adventofcode.com/) puzzles, written in Rust and run from a single command-line binary. Each year is a module, so new events slot in alongside the old ones.

## Overview

Each puzzle day lives in its own module with a `part1` and `part2` function. Puzzle input is embedded into the binary at compile time with `include_str!("input.txt")`, so the program needs no arguments beyond the year and, optionally, the day. Each part prints its result to standard output.

Years currently in the crate: 2024, 2025 and 2026, the last scaffolded and ready for December.

## What's implemented

| Event | Days | Notes |
| --- | --- | --- |
| 2025 | 1 to 11 | Both parts solved. |
| 2025 | 12 | Part 1 only. It prints a lower and upper bound on the answer, from area checks, rather than packing the shapes exactly. |
| 2024 | 1 to 20 | Both parts solved. |
| 2024 | 21 to 25 | Stubs that print "not implemented yet". |
| 2026 | 1 to 12 | Stubs ready for the 2026 event. |

Some solutions use external crates: Z3 (through the `z3` crate) solves the integer systems in 2024 day 13 and 2025 day 10, and `ordered-float` is used in 2025 day 8.

## Tech stack

- Rust, 2024 edition
- [`regex`](https://crates.io/crates/regex) for input parsing
- [`z3`](https://crates.io/crates/z3) bindings to the Z3 SMT solver
- [`ordered-float`](https://crates.io/crates/ordered-float) and [`once_cell`](https://crates.io/crates/once_cell)

## Getting started

### Prerequisites

- A Rust toolchain that supports the 2024 edition (Rust 1.85 or later)
- The Z3 library and headers, plus Clang, for the `z3` crate to build and link against (for example `brew install z3` on macOS or `apt install libz3-dev clang` on Debian and Ubuntu)
- Your own puzzle input for each day, saved as `input.txt` in that day's directory (for example `src/aoc2025/day7/input.txt`). Because inputs are embedded at compile time, the build fails if one is missing.

### Build

```sh
cargo build --release
```

## Usage

```sh
cargo run --release -- <year>          # run every day of a year
cargo run --release -- <year> <day>    # run one day
cargo run --release -- all             # run every day of every year
```

Examples:

```sh
cargo run --release -- 2025       # all 2025 days
cargo run --release -- 2025 7     # 2025 day 7
cargo run --release -- 2024 15    # 2024 day 15
```

Running with no arguments prints the usage text.

## Project structure

```
src/
├── main.rs            # argument parsing; YEARS lists each year's run_day / run_all
├── aoc2024/
│   ├── mod.rs         # run_day / run_all for 2024 (days 1-25)
│   └── dayN/
│       ├── dayN.rs    # part1, part2 and the dayN entry point
│       └── input.txt  # puzzle input, embedded at compile time
├── aoc2025/           # days 1-12, same layout
└── aoc2026/           # days 1-12, same layout
```

Some day directories also contain `test_input.txt` files with smaller sample inputs; the code does not load them.

## Adding a year

1. Copy the layout of an existing year to `src/aocYYYY/`, with a `mod.rs` exposing `run_day` and `run_all`.
2. Add `mod aocYYYY;` and an entry in `YEARS` in `src/main.rs`.

Advent of Code has 12 puzzles a year from 2025 onwards, so `aoc2026` has days 1 to 12. Each 2026 day has an empty `input.txt` so the crate builds; paste in your input on the day.

## Status and limitations

- 2024 days 21 to 25 and all of 2026 are placeholders.
- 2025 day 12 reports a range rather than a single answer, and has no part 2.
- There are no automated tests; solutions are checked by running them.
- `Cargo.toml` declares `clipboard` and `sound` features, but no code uses them yet.

## Acknowledgements

Puzzles by Eric Wastl at [Advent of Code](https://adventofcode.com/).
