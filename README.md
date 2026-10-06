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
  `teru` rather than `ls` for that reason. There is no fixed naming convention
  yet — it will be settled once there are more tools to judge it by.

## Tools

| Crate | Binary | Notes |
| --- | --- | --- |
| [`crates/teru`](crates/teru) | `teru` | A take on `ls`. Supports plain listing, `-a`, and `-l`. |

## Usage

```console
# Build the whole workspace
cargo build --workspace

# Run a single tool
cargo run -p teru

# Tests and lints, as mise tasks. CI and the git hooks run these same tasks,
# so the commands behind them are defined once, in mise.toml.
mise run test
mise run lint        # reports; never rewrites
mise run fmt         # rewrites in place (`mise run fmt-check` only reports)
```

## Development setup

The Rust toolchain is pinned in `rust-toolchain.toml`, so `cargo` picks up the
right compiler on its own — no setup needed for the commands above. Note that
this is a different thing from `rust-version` in `Cargo.toml`, which declares
the minimum supported Rust version and is deliberately left behind the pinned
toolchain.

Everything else is pinned in `mise.toml` and installed with
[mise](https://mise.jdx.dev):

```console
mise install
```

### Git hooks

The same checks run from git hooks, so a red build is caught before it reaches
CI. `.git/hooks` is outside version control, so the hooks live in `.githooks/`
instead and are enabled per clone:

```console
git config core.hooksPath .githooks
```

- `pre-commit` formats the workspace and re-stages the result, then runs
  Clippy. Formatting is applied because there is nothing to learn from running
  rustfmt by hand; lint violations are only reported, because there is.
- `pre-push` runs formatting, Clippy and the test suite. The slow part lives
  here rather than in `pre-commit` so that committing stays quick.

`git commit --no-verify` and `git push --no-verify` skip them.

Two things worth knowing before relying on them:

- **They see the working tree, not the index.** The `pre-commit` hook re-stages
  the `.rs` files that were already staged, so if you staged part of a file with
  `git add -p`, the rest of that file is swept into the commit as well. Clippy
  and the tests likewise read what is on disk. Stage whole files, or expect the
  two to differ.
- **Hooks inherit the environment of whatever invoked git.** A terminal has mise
  on `PATH`; an IDE launched from the desktop often does not, and the hook then
  fails with a message saying so rather than skipping the checks silently.

## Supply chain

GitHub Actions are pinned to full commit SHAs rather than tags. Tags are
mutable, and repointing a widely used tag at malicious code is an attack that
has already happened in the wild. [pinact](https://github.com/suzuki-shunsuke/pinact)
maintains the pins:

```console
# Pin, or re-pin after adding an action
pinact run

# What CI enforces: every action pinned, every version comment accurate
pinact run --check --verify-comment
```

The workflow also restricts `GITHUB_TOKEN` to `contents: read`, which bounds
what a compromised action could do with it.

Pinning freezes versions, and they are bumped by hand when needed. Renovate is
configured in `.github/renovate.json` but switched off with `"enabled": false`;
remove that line to bring automatic update pull requests back.

## Releases

Each crate in `crates/` is versioned and released independently. A crate
opts out with `publish = false` in its `Cargo.toml` (`common` did, before it
was removed for having no content yet).

[release-plz](https://release-plz.dev) drives this from
`.github/workflows/release-plz.yml`, reading
[Conventional Commits](https://www.conventionalcommits.org/) (see
`CLAUDE.md`) to decide what changed:

- Every push to `main` opens or updates a PR that bumps the version and
  changelog of whichever crates changed.
- Merging that PR tags the commit, publishes the crate(s) to crates.io, and
  creates a GitHub Release with the generated notes.

**Merge the release PR with a regular merge commit, not squash-merge.**
release-plz releases by checking out the release PR's last commit; squash
merging replaces it with a new commit GitHub creates, so release-plz falls
back to releasing whatever is on `main` instead of what was actually
reviewed. Harmless for a single maintainer with no merge queue, but the
regular-merge habit costs nothing and avoids depending on that. Ordinary
(non-release) PRs are unaffected — squash-merge those as usual.

Publishing uses crates.io's [Trusted Publishing](https://crates.io/docs/trusted-publishing)
(OIDC) rather than a stored API token: the workflow proves its identity
directly, so there is no long-lived secret to leak or rotate. This only works
for versions *after* a crate's first release, though — crates.io requires a
new crate's very first version to be published by hand, once, with
`cargo publish`.

## License

Licensed under either of:

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE))
- MIT License ([`LICENSE-MIT`](LICENSE-MIT))

at your option.
