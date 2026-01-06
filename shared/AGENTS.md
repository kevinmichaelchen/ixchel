# Shared Crates

Shared infrastructure for helix-tools. These crates provide common functionality used across multiple tools.

## Crates

| Crate | Purpose | Consumers | Status |
|-------|---------|-----------|--------|
| [helix-config](./helix-config/) | Hierarchical config loading | All tools | ✅ Active |
| [helix-id](./helix-id/) | Hash-based ID generation | hbd, helix-decisions | ✅ Active |
| [helix-storage](./helix-storage/) | Trait-based vector storage (JSON) | helix-decisions | ⚠️ Deprecated (see below) |
| [helix-embeddings](./helix-embeddings/) | Semantic embeddings via fastembed | helix-decisions, hbd, helix-docs | ✅ Active |
| [helix-discovery](./helix-discovery/) | Git root and project marker discovery | helix-decisions, hbd | ✅ Active |

## helix-storage Deprecation

> **Status:** helix-storage is being superseded by native HelixDB integration.

### Background

helix-storage was created as a temporary abstraction for MVP development:
- `VectorStorage<T>` trait for vector + metadata storage
- `JsonFileBackend` implementation (simple JSON persistence)
- Works, but doesn't scale for large repos (100+ items)

### Migration Path

Tools are migrating to **native HelixDB** for:
- 3-stage incremental indexing (stat → hash → embed)
- Native graph traversal (edges, not in-memory)
- LMDB persistence (no re-scanning on restart)

| Tool | Current | Target | Status |
|------|---------|--------|--------|
| helix-decisions | helix-storage (JSON) | HelixDB native | 🚧 Phase 3 planned |
| hbd | File-based (no helix-storage) | HelixDB native | 🚧 Planned |
| helix-docs | Not started | HelixDB native | 📋 Backlog |

### What This Means for Consumers

1. **helix-decisions (Phase 3):** Will replace `PersistentDecisionStorage` with `HelixDecisionStorage`
2. **New tools:** Should use HelixDB directly, not helix-storage
3. **Existing tools:** Can continue using helix-storage; it remains functional

### HelixDB API Patterns

When integrating HelixDB, follow the corrected patterns in:
- `helix-decisions/docs/phase3/PHASE_3_CORRECTIONS.md`
- `helix-decisions/docs/phase3/CORRECTIONS_QUICK_REFERENCE.txt`

Key requirements:
- **Edges:** Write to 3 databases (edges_db, out_edges_db, in_edges_db)
- **Nodes:** Use arena allocation + ImmutablePropertiesMap
- **Vectors:** Stored separately, linked via vector_id property
- **Keys:** Use `hash_label()` for adjacency DB keys

## Design Principles

1. **Loose coupling** — Tools depend on traits, not implementations
2. **Project-local by default** — Data lives in the repo (`.helix/data/` or `.helixdb/`), not globally
3. **Config-driven** — Behavior configured via `~/.helix/config/` and `.helix/`
4. **Minimal dependencies** — Each crate depends only on what it needs
5. **Prefer native HelixDB** — For new graph/vector storage, use HelixDB directly

## Adding a New Shared Crate

1. Create directory: `shared/helix-{name}/`
2. Add `Cargo.toml`, `README.md`, `src/lib.rs`
3. Add to workspace `Cargo.toml` members
4. Update this AGENTS.md

## See Also

- `helix-decisions/specs/design.md` - Phase 3 HelixDB architecture
- `helix-decisions/docs/phase3/` - Detailed implementation plans
