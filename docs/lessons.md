# Lessons

Review points that could not be turned into a lint — design decisions, naming,
documentation habits. Lint-able points belong in `[workspace.lints]`,
`clippy.toml`, or a custom `dylint` lint instead; see `CLAUDE.md`.

Each entry: what was pointed out, why it matters, and what to do instead.

<!-- Append new entries below. -->

## Bind a repeated method-call result to a local variable

`crates/teru/src/format.rs`, `format_permissions`: called `metadata.mode()` nine
times (once per permission bit check) instead of binding it once with
`let mode = metadata.mode();` at the top of the function.

Why it matters: the call itself is cheap (a struct field read), so this is
not a performance issue. It is a readability one — repeating the same
expression obscures that all nine checks operate on one fixed value, and
makes the checks themselves noisier (`metadata.mode() & 0o400` vs.
`mode & 0o400`).

What to do instead: when a function reads the same value from an expression
three or more times, bind it to a local variable once at the top, even if
the expression has no side effects to worry about.
