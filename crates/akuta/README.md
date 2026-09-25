# akuta

An `rm` replacement that moves files to the trash instead of deleting them,
for learning Rust.

The name comes from the Japanese 芥 (*akuta*, "rubbish, dust").

> **Status: work in progress.** The skeleton is in place and the core
> functions are still `todo!()`, so the test suite fails by design until
> they are filled in. The crate is `publish = false` until then.

## Usage

```console
akuta <path>...            # move paths to the trash
akuta --list               # list trashed entries, newest first
akuta --restore <name>     # restore one entry by its name in the trash
```

It is meant to be used as `alias rm=akuta`: `rm`'s flags (`-r`, `-f`, `-rf`,
`--recursive`, …) are accepted and ignored, and positional arguments are
always paths.

Exit code matches GNU `rm`: `0` on success, `1` on any failure.

## Storage

The trash follows the [freedesktop.org Trash specification](https://specifications.freedesktop.org/trash-spec/latest/):
`$XDG_DATA_HOME/Trash` (default `~/.local/share/Trash`), with the entries in
`files/` and one `.trashinfo` per entry in `info/`. It is **not** macOS's
`~/.Trash`, and Finder's "Put Back" does not know about it.

The location can be overridden in `$XDG_CONFIG_HOME/akuta/config` (default
`~/.config/akuta/config`):

```
trash_dir = /absolute/path/to/Trash
```

## Scope

The full specification, including what is deliberately left out, is
[issue #41](https://github.com/RikiyaOta/utils/issues/41). In short:

- Home trash only. No cross-device moves: if `rename` fails, that is the
  error.
- `DeletionDate` is written in UTC, not local time. The standard library
  cannot read the system time zone.
- Arguments must be valid UTF-8, as in `teru`.
- Unix-only (macOS and Linux).

See the repository [README](../../README.md) for how this crate fits into
the workspace, and [CLAUDE.md](../../CLAUDE.md) for how it is developed.
