# teru

A reimplementation of the Unix `ls` command, for learning Rust.

The name comes from the Japanese 照らす (*terasu*, "to shine a light on, to
illuminate") — it lights up what is inside a directory.

## Usage

```console
cargo run -p teru           # plain listing, current directory only
cargo run -p teru -- -a     # include hidden (dotfile) entries
cargo run -p teru -- -l     # long listing: permissions, size, modified time
cargo run -p teru -- -a -l  # combine flags (see "Scope" below)
```

Example `-l` output:

```
drwxr-xr-x 128 2026-08-31 02:24 crates
-rw-r--r-- 1514 2026-08-28 11:42 Cargo.toml
```

Exit code matches the local `ls`: `0` on success, `1` if a directory cannot
be read.

## Scope

This covers the scope of [issue #11](https://github.com/RikiyaOta/utils/issues/11):
plain listing, `-a` (show hidden entries), and `-l` (long listing). A few
things are deliberately left out, all for the same reason: **standard
library only, no external crates**.

- `-l` shows permissions, size, and modification time, but not owner/group
  *names* or the hard-link count. Resolving a uid to a username needs
  `libc`; the standard library alone cannot do it.
- Modification times are shown in UTC, not the local timezone. Local time
  needs the system's timezone database (IANA `tzdata`), which the standard
  library does not expose and which would take real effort to parse
  correctly from scratch.
- Short flags cannot be combined: `-a -l` works, `-al` does not. The
  argument parser only recognises exact `-a` / `-l` tokens.
- Unix-only. Permission bits and file metadata come from
  `std::os::unix::fs`, so this crate does not build on Windows.

See the repository [README](../../README.md) for how this crate fits into
the workspace, and [CLAUDE.md](../../CLAUDE.md) for how it is developed.
