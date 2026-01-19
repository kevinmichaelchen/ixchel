# IX-DAEMON AGENTS

**Parent:** See `../AGENTS.md` for workspace context.

## Overview

IPC daemon and client helpers for background sync and single-writer enforcement.
Provides the `ixcheld` binary.

## Structure

```
ix-daemon/
├── src/
│   ├── lib.rs             # Library exports
│   ├── client.rs          # IPC client
│   ├── server.rs          # IPC server
│   ├── queue.rs           # Queue and coalescing
│   └── bin/ixcheld.rs      # Daemon binary
├── specs/                 # requirements/design/tasks
└── tests/
```

## Where To Look

| Task              | Location                       |
| ----------------- | ------------------------------ |
| Daemon entrypoint | `ix-daemon/src/bin/ixcheld.rs` |
| IPC server        | `ix-daemon/src/server.rs`      |
| IPC client        | `ix-daemon/src/client.rs`      |
| Queue behavior    | `ix-daemon/src/queue.rs`       |
