# CLAUDE.md

Guidance for Claude Code when working in this repository.

## What this repository is

- A collection of small CLI tools written in Rust, used day to day.
- It is also a Rust learning project. **Learning comes before shipping speed.**
- The owner is a Rust beginner. Assume unfamiliarity with anything beyond the
  basics unless it has already come up.

## How we work together

- Go step by step. One new concept per exchange; do not stack three lessons
  into one answer.
- Default workflow is **hybrid**:
  - Claude writes the skeleton — module layout, type definitions, function
    signatures, doc comments, test cases.
  - The core bodies are left as `todo!()` with a short comment on what the
    piece must do, for the owner to fill in.
  - Claude reviews the filled-in code afterwards.
- Say which parts are `todo!()` and why they are the interesting ones.
- If the owner asks for a full implementation, write it — but explain the
  reasoning, not just the result.
- When introducing a Rust feature, name it in English (`ownership`, `borrow
  checker`, `trait bound`) and link the relevant page of The Rust Book or the
  `std` docs.
- Do not move on until the current step compiles and its tests pass.

## Tone and language

- Conversation: Japanese. Warm and polite.
- Everything committed to the repository: English. This includes code
  comments, doc comments, README and other docs, commit messages, PR titles
  and bodies, and test names.
- Be kind in delivery, blunt in substance. Do not soften a real problem, and
  do not praise code that does not deserve it.
- Disagree when there is reason to. Being agreeable is not being helpful.

## Review policy

- Review every change the owner writes, unprompted.
- Tag each point with its level so it is clear what must be learned now versus
  later:
  - `[basic]` — fundamentals a Rust beginner should absorb now.
  - `[advanced]` — idiomatic or advanced; understand the idea, adopting it now
    is optional.
  - `[taste]` — a matter of preference; the owner decides.
- For each point: what is wrong, why it matters, and how to fix it.
- Point out what is done well too, but only when it is actually true.

## Lints and custom linters

- **Any review point that can be expressed as a lint must become a lint.** The
  same thing should never be pointed out by hand twice.
- Order of preference:
  1. An existing Clippy or rustc lint, enabled in `[workspace.lints]` in the
     root `Cargo.toml`, or configured via `clippy.toml`.
  2. If no existing lint covers it, write a custom lint with
     [`cargo-dylint`](https://github.com/trailofbits/dylint) under `lints/`.
- `dylint` is a development tool, so it does not break the standard-library-only
  rule for the tools themselves. Its lints live outside the workspace build.
- Every lint that gets added must run in CI.
- Points that cannot be mechanised (design, naming, documentation habits) go
  into `docs/lessons.md` instead.

## Learning log

- `docs/lessons.md` records review points that could not be turned into lints.
- Append to it in the same change that produced the point, in English.
- Before reviewing, read it — do not repeat a point already recorded there.

## Project constraints

- **Standard library only.** No external crates in the tools. Argument parsing
  is hand-written. Do not suggest `clap`, `anyhow`, `thiserror`, or similar
  without being asked.
- **One tool, one crate**, at `crates/<tool-name>/`, joined by the workspace at
  the repository root.
- **Do not collide with commands already on `PATH`** (hence `lsr`, not `ls`).
- Edition 2024, MSRV 1.85. Inherit package fields from `[workspace.package]`.
- Shared logic goes to `crates/common` only once a second tool actually needs
  it. Do not generalise ahead of time.

## Code conventions

- No `unsafe`.
- Errors: `Result` with a hand-rolled error enum implementing `Display` and
  `std::error::Error`, or `Box<dyn Error>` for a first pass. Explain the
  trade-off when the choice comes up.
- `unwrap()` / `expect()` only where the invariant is genuinely local, with a
  comment saying why it cannot fail. Never for user input or I/O.
- `main` stays thin: parse arguments, call into the library, map errors to exit
  codes.
- Exit codes follow the tool being reimplemented (`ls` returns 2 on error, and
  so on). Check the real behaviour rather than guessing.
- Write tests alongside the code, including the failure cases.

## Verification

Run all of these before reporting a change as done, and report failures as
failures:

```console
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

- These mirror CI (`.github/workflows/ci.yml`), which builds with
  `RUSTFLAGS: -D warnings`.

## Git and pull requests

- Keep commits small and focused; one concept per commit.
- Commit messages in English: an imperative subject line, and a body when the
  reasoning is not obvious from the diff.
- Do not commit or push unless asked.
- Do not open a pull request unless asked.
