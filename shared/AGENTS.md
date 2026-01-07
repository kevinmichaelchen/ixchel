# Shared Crates

Shared infrastructure for helix-tools. These crates provide common functionality used across multiple tools.

## Architecture: Trait-Based Storage

> **Key Decision:** Shared crates do NOT depend on HelixDB.
>
> See [ADR-004](../.decisions/004-trait-based-storage-architecture.md) for the full rationale.

Each tool defines its own storage trait. HelixDB is an implementation detail, not a shared dependency.

```
shared/helix-embeddings     → Returns Vec<f32>, no storage knowledge
shared/helix-config         → Documents paths, no HelixDB dependency
shared/helix-id             → Pure utility
shared/helix-discovery      → Pure utility
shared/helix-daemon         → IPC protocol + client helpers, no HelixDB dependency

helix-decisions/            → Defines DecisionStore trait
                            → Provides HelixDB implementation
hbd/                        → Defines IssueStore trait
                            → Provides HelixDB implementation
```

## Crates

| Crate | Purpose | HelixDB Dependency | Specs |
|-------|---------|-------------------|-------|
| [helix-config](./helix-config/) | Hierarchical config loading | NO | [specs](./helix-config/specs/) |
| [helix-id](./helix-id/) | Hash-based ID generation | NO | [specs](./helix-id/specs/) |
| [helix-embeddings](./helix-embeddings/) | Semantic embeddings via fastembed | NO | [specs](./helix-embeddings/specs/) |
| [helix-discovery](./helix-discovery/) | Git root and project marker discovery | NO | [specs](./helix-discovery/specs/) |
| [helix-daemon](./helix-daemon/) | Global IPC protocol + client helpers | NO | [specs](./helix-daemon/specs/) |
| [helix-storage](./helix-storage/) | ~~Trait-based vector storage~~ | **REMOVE** | [specs](./helix-storage/specs/) |

## helix-storage Removal

helix-storage was scaffolding. Each tool now defines its own storage trait:
- **helix-decisions** → `DecisionStore` trait with HelixDB backend
- **hbd** → `IssueStore` trait with HelixDB backend

Do not add new dependencies on helix-storage.

## Design Principles

1. **No HelixDB in shared crates** — Shared crates are pure utilities
2. **Trait-based storage** — Each tool owns its storage interface
3. **Loose coupling** — High-level modules don't depend on low-level modules
4. **Testability** — Tools can use in-memory backends for tests
5. **Project-local by default** — Data lives in `.helix/data/`, not globally

## For Tools Using HelixDB

HelixDB's LMDB provides built-in persistence. When you open a HelixDB environment at a path, it creates `data.mdb` and `lock.mdb` automatically. Data persists across runs.

For HelixDB API patterns, see:
- `helix-decisions/docs/phase3/PHASE_3_CORRECTIONS.md`
- `helix-decisions/docs/phase3/CORRECTIONS_QUICK_REFERENCE.txt`

## Adding a New Shared Crate

1. Create directory: `shared/helix-{name}/`
2. Add `Cargo.toml`, `README.md`, `src/lib.rs`
3. Create `specs/` with `design.md` and `requirements.md`
4. **Ensure no HelixDB dependency** — Keep it pure
5. Add to workspace `Cargo.toml` members
6. Update this AGENTS.md

## See Also

- [ADR-004](../.decisions/004-trait-based-storage-architecture.md) — Trait-based storage architecture
- `helix-decisions/specs/design.md` — Example of tool with storage trait
