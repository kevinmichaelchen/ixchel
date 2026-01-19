# IX-CONFIG AGENTS

**Parent:** See `../../AGENTS.md` for workspace context.

## Overview

Hierarchical configuration loader for Ixchel. Handles global configs,
project overrides, and environment variable mapping.

## Structure

```
crates/ix-config/
├── src/lib.rs             # Config loader and path helpers
└── specs/                 # requirements/design
```

## Where To Look

| Task                  | Location                      |
| --------------------- | ----------------------------- |
| Config load behavior  | `crates/ix-config/src/lib.rs` |
| Config hierarchy docs | `crates/ix-config/README.md`  |
