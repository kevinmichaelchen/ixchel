# ix-daemon

Global per-user daemon for Ixchel. Provides a shared IPC layer for background
sync, single-writer enforcement, and queueing across repos and tools.

## Responsibilities

- Accept IPC requests over local sockets
- Namespace requests by `{repo_root, tool}`
- Enforce per-repo writer locks
- Queue and coalesce sync work
- Report status for `--sync` and health checks

## IPC

- Unix socket: `~/.ixchel/run/ixcheld.sock`
- Windows: named pipe (path defined in specs)

## Specs

- [Requirements][requirements]
- [Design][design]
- [Tasks][tasks]

<!-- Links -->

[requirements]: specs/requirements.md
[design]: specs/design.md
[tasks]: specs/tasks.md
