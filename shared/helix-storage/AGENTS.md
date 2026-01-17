# HELIX-STORAGE AGENTS

**Parent:** See `../../AGENTS.md` and `../AGENTS.md` for shared context.

## Overview

Deprecated storage abstraction kept for reference. New tools should avoid adding
new dependencies here.

## Structure

```
shared/helix-storage/
├── src/lib.rs             # Legacy storage traits/backends
└── specs/                 # requirements/design (deprecation rationale)
```

## Where To Look

| Task               | Location                               |
| ------------------ | -------------------------------------- |
| Legacy storage API | `shared/helix-storage/src/lib.rs`      |
| Deprecation notes  | `shared/helix-storage/specs/design.md` |
