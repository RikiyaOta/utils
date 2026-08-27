# utils

A collection of small command-line tools I write for myself, in Rust.

Some of them reimplement existing commands; others exist simply because I want
them. Either way, writing them is also how I learn the language.

## Goals

- Get hands-on practice with Rust and its standard library
- Work through what a small but usable CLI actually needs: file I/O, path
  handling, error handling, exit codes
- Where a tool has an existing counterpart, read its behaviour closely and
  reproduce it piece by piece

## Principles

- **Standard library only.** Learning is the point, so no external crates as a
  rule. Argument parsing is hand-written too (`clap` and friends are something
  to reconsider later, if it ever becomes worth it).
- **One tool, one crate.** Each tool lives in `crates/<tool-name>/` as its own
  crate, tied together by the Cargo workspace at the repository root.
- **Don't collide with commands already on `PATH`.** The first tool is named
  `lsr` rather than `ls` for that reason. There is no fixed naming convention
  yet — it will be settled once there are more tools to judge it by.

## Tools

| Crate | Binary | Notes |
| --- | --- | --- |
| [`crates/lsr`](crates/lsr) | `lsr` | A take on `ls`. Scaffolding only so far. |
| [`crates/common`](crates/common) | (library) | Empty. Shared logic moves here once there is any. |

## Usage

```console
# Build the whole workspace
cargo build --workspace

# Run a single tool
cargo run -p lsr

# Tests and lints
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE))
- MIT License ([`LICENSE-MIT`](LICENSE-MIT))

at your option.
