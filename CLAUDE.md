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

## Lints

- **Any review point that can be expressed as a lint must become a lint.** The
  same thing should never be pointed out by hand twice.
- Reach for the upstream tooling before writing anything. Clippy ships more
  than 800 lints; `restriction` (~130 of them) exists precisely to enforce
  project policy, and is opt-in lint by lint. Search
  [the lint list](https://rust-lang.github.io/rust-clippy/master/) first.
- Order of preference:
  1. An existing Clippy or rustc lint, enabled in `[workspace.lints]` in the
     root `Cargo.toml`.
  2. A `clippy.toml` setting. `disallowed-methods`, `disallowed-types` and
     `disallowed-macros` ban a specific path outright, and thresholds such as
     `too-many-lines` are tunable — no lint code required. This covers most of
     what looks at first like it needs a bespoke lint.
  3. Only if both fail: a custom lint via
     [`cargo-dylint`](https://github.com/trailofbits/dylint) under `lints/`.
     Expect this never to happen. It costs a pinned nightly toolchain and
     compiler-internals knowledge, so propose it explicitly and get agreement
     before starting.
- Points that cannot be mechanised (design, naming, documentation habits) go
  into `docs/lessons.md` instead.
- Every lint that gets enabled must run in CI.
- **When a lint takes over a convention written down in prose, delete the
  prose in the same change**, and let the lint's comment in `Cargo.toml` carry
  the reasoning. A rule described in two places is a rule that will disagree
  with itself.

### Working with lint configuration

- When enabling a group in `[workspace.lints.clippy]`, give it `priority = -1`
  so that individual lints listed after it can override it.
- Never enable the whole `restriction` group — the lints in it contradict each
  other by design. Pick them one at a time.
- Do not deny `clippy::todo`. The hybrid workflow above depends on `todo!()`.
- Prefer `#[expect(lint, reason = "…")]` over `#[allow(…)]`: it warns once the
  exception stops being needed. Every exception carries a reason.
- When a lint is enabled, say in the commit message which review point it
  replaces.

### Contributing lints upstream

- Sending a new lint to Clippy itself is a good Rust-community goal, and a good
  exercise once the basics are comfortable — `cargo dev new_lint` and the
  Clippy book are the entry points.
- It is **not** this repository's mechanism for preventing repeat review
  points. Clippy only accepts lints that generalise beyond one project, and the
  path from merge to a stable release this repository can rely on takes months.
- So treat it as a separate learning track. Never let a repo-level convention
  wait on an upstream pull request.

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
- **Do not collide with commands already on `PATH`** (hence `teru`, not `ls`).
- Edition 2024, MSRV 1.85. Inherit package fields from `[workspace.package]`.
- Shared logic goes to `crates/common` only once a second tool actually needs
  it. Do not generalise ahead of time.

## Code conventions

Conventions that a lint already enforces are **not** repeated here — see
`[workspace.lints]` in the root `Cargo.toml`, where each entry carries the rule
it stands for. Two sources of truth drift apart, and only one of them runs.
What follows is what no lint can check.

- Errors: `Result` with a hand-rolled error enum implementing `Display` and
  `std::error::Error`, or `Box<dyn Error>` for a first pass. Explain the
  trade-off when the choice comes up.
- `main` parses arguments, calls into the library, and maps errors to exit
  codes. Logic lives below it, not in it.
- Exit codes follow the tool being reimplemented (`ls` returns 2 on error, and
  so on). Check the real behaviour rather than guessing.
- Write tests alongside the code, including the failure cases.
- To step outside an enforced convention, write
  `#[expect(clippy::the_lint, reason = "…")]` on the narrowest possible scope.
  The reason is the comment that would otherwise have been asked for in review.

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
- Commit messages in English, as [Conventional Commits](https://www.conventionalcommits.org/):
  an imperative subject line prefixed with a type, and a body when the
  reasoning is not obvious from the diff. release-plz
  (`.github/workflows/release-plz.yml`) reads the type to decide each crate's
  next version and to write its changelog, so picking the right one is not
  just style:
  - `feat:` — user-visible new behaviour. Bumps the minor version.
  - `fix:` — a bug fix. Bumps the patch version.
  - `feat!:` / a `BREAKING CHANGE:` footer — an incompatible change. Bumps
    the major version.
  - `docs:`, `refactor:`, `test:`, `ci:`, `chore:` — no version bump; grouped
    separately (or omitted) in the changelog.
- Do not commit or push unless asked.
- Do not open a pull request unless asked.
