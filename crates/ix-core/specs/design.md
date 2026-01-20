# Design

**Crate:** `ix-core`\
**Purpose:** Ixchel core library (git-first Markdown domain)

This document describes the high-level design of `ix-core`.

## Overview

`ix-core` is the shared domain library behind Ixchel apps (`ix-cli`, `ix-mcp`).
It owns:

- Repository discovery and `.ixchel/` layout (`IxchelRepo`, `IxchelPaths`)
- Markdown parsing/rendering + frontmatter helpers
- Entity kinds + id-prefix semantics
- Relationship inference + repository integrity checks
- A backend-agnostic indexing interface (`IndexBackend`)

`ix-core` is intentionally adapter-free: concrete storage backends (HelixDB, etc.)
implement `IndexBackend` in separate crates.

## On-Disk Canonical Layout

Ixchel’s canonical source of truth is Markdown under `.ixchel/`:

```
.ixchel/
  config.toml
  decisions/
  issues/
  ideas/
  reports/
  sources/
  citations/
  agents/
  sessions/
  data/    # rebuildable cache (gitignored)
  models/  # embedding models (gitignored)
```

## Core Types

- `IxchelRepo`: entrypoint for reading/writing entities and validating repos.
- `IxchelPaths`: consistent path computation relative to git root.
- `EntityKind`: typed entity kinds + directory name and id-prefix mapping.
- `MarkdownDocument`: parsed frontmatter + body representation.
- `IndexBackend`: port trait for rebuildable cache implementations.

## Relationship Inference

Ixchel treats frontmatter keys (other than known metadata keys) as relationship
labels. Values are interpreted as relationship targets only when they look like a
canonical id (`<prefix>-<6..12 hex>`). This avoids incorrectly treating metadata
like `labels: [bug]` as graph edges while still allowing validation of unknown
prefixes (`foo-123456`) during `check()`.

## Tag Aggregation

The `tags` frontmatter field is reserved for free-form labels. `ix-core` provides:

- `collect_tags(kind)`: Scans all entities (or a specific kind), returns
  `HashMap<String, Vec<EntityId>>`. Tag identity is case-sensitive and based on
  trimmed tag values; empty tags are ignored and duplicates within a single
  entity count once.
- `list_untagged(kind)`: Returns entities with no tags (missing tag field or
  only empty/whitespace values) for the full repo or a specific kind.
- `add_tags(id, tags)` / `remove_tags(id, tags)`: Mutates entity frontmatter
  with idempotent tag adds/removals and updates `updated_at` on changes.

This enables CLI/MCP tools to expose the tag vocabulary. LLMs are smart enough to
detect similarity and synonyms themselves—no need for built-in fuzzy matching.

## List Sorting

The `list()` function accepts an optional `ListSort` enum controlling result order:

| Sort Option             | Behavior                                              |
| ----------------------- | ----------------------------------------------------- |
| `CreatedDesc` (default) | Sort by `created_at` descending (newest first)        |
| `UpdatedDesc`           | Sort by `updated_at` descending (recently modified first) |

Timestamps are parsed from frontmatter. Entities missing timestamps sort after
entities with valid dates (treated as epoch 0).

## Compatibility Notes

- `bd-*` ids (legacy issue ids) are accepted as `EntityKind::Issue` for
  migration/compatibility, while `iss-*` remains the canonical Ixchel issue prefix.
