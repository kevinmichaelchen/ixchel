# IX-CONFIG AGENTS

**Parent:** See `../AGENTS.md` for workspace context.

## Overview

Hierarchical configuration loader for Ixchel. Handles global configs,
project overrides, and environment variable mapping.

## Structure

```
ix-config/
├── src/lib.rs             # Config loader and path helpers
└── specs/                 # requirements/design
```

## Where To Look

| Task                  | Location               |
| --------------------- | ---------------------- |
| Config load behavior  | `ix-config/src/lib.rs` |
| Config hierarchy docs | `ix-config/README.md`  |
