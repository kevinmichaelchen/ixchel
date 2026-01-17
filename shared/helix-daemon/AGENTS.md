# HELIX-DAEMON AGENTS

**Parent:** See `../../AGENTS.md` and `../AGENTS.md` for shared context.

## Overview

IPC daemon and client helpers for background sync and single-writer enforcement.
Provides the `helixd` binary.

## Structure

```
shared/helix-daemon/
├── src/
│   ├── lib.rs             # Library exports
│   ├── client.rs          # IPC client
│   ├── server.rs          # IPC server
│   ├── queue.rs           # Queue and coalescing
│   └── bin/helixd.rs       # Daemon binary
├── specs/                 # requirements/design/tasks
└── tests/
```

## Where To Look

| Task              | Location                                |
| ----------------- | --------------------------------------- |
| Daemon entrypoint | `shared/helix-daemon/src/bin/helixd.rs` |
| IPC server        | `shared/helix-daemon/src/server.rs`     |
| IPC client        | `shared/helix-daemon/src/client.rs`     |
| Queue behavior    | `shared/helix-daemon/src/queue.rs`      |
