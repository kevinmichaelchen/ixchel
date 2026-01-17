# HELIX-EMBEDDINGS AGENTS

**Parent:** See `../../AGENTS.md` and `../AGENTS.md` for shared context.

## Overview

Wrapper around fastembed with config-driven model selection. Provides the
`Embedder` API used by other tools.

## Structure

```
shared/helix-embeddings/
├── src/lib.rs             # Embedder API and config
└── specs/                 # requirements/design
```

## Where To Look

| Task              | Location                             |
| ----------------- | ------------------------------------ |
| Embedder behavior | `shared/helix-embeddings/src/lib.rs` |
| Model/config docs | `shared/helix-embeddings/README.md`  |
