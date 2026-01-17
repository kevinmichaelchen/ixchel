# HELIX-CONFIG AGENTS

**Parent:** See `../../AGENTS.md` and `../AGENTS.md` for shared context.

## Overview

Hierarchical configuration loader for helix-tools. Handles global configs,
project overrides, and environment variable mapping.

## Structure

```
shared/helix-config/
├── src/lib.rs             # Config loader and path helpers
└── specs/                 # requirements/design
```

## Where To Look

| Task                  | Location                         |
| --------------------- | -------------------------------- |
| Config load behavior  | `shared/helix-config/src/lib.rs` |
| Config hierarchy docs | `shared/helix-config/README.md`  |
