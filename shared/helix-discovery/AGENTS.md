# HELIX-DISCOVERY AGENTS

**Parent:** See `../../AGENTS.md` and `../AGENTS.md` for shared context.

## Overview

Finds git roots and project markers (e.g. `.decisions/`, `.tickets/`, `.helix/`).

## Structure

```
shared/helix-discovery/
├── src/lib.rs             # Discovery helpers and errors
└── specs/                 # requirements/design
```

## Where To Look

| Task                | Location                            |
| ------------------- | ----------------------------------- |
| Discovery algorithm | `shared/helix-discovery/src/lib.rs` |
| Usage examples      | `shared/helix-discovery/README.md`  |
