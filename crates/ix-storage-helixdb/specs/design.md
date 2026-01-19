# Design

**Crate:** `ix-storage-helixdb`\
**Purpose:** HelixDB-backed rebuildable cache for Ixchel

## Overview

`ix-storage-helixdb` implements `ix_core::index::IndexBackend` by mapping Ixchel
Markdown entities to HelixDB:

- Nodes represent entities (`id`, `kind`, `title`, `status`, `tags`, `body`, etc.)
- Edges represent relationships inferred from frontmatter
- Vectors represent semantic embeddings for hybrid/semantic search

The database directory is rebuildable and safe to delete. Ixchel can reconstruct
it from Markdown via `ixchel sync`.

## Pathing

The DB path is computed as:

```
<git root>/.ixchel/<storage.path>
```

By default: `.ixchel/data/ixchel/`.

## Sync Strategy (Current)

- Full rebuild on each sync (clear + reinsert)
- Relationship edges are created after all nodes are inserted

## Search

- Query embedding via `fastembed`
- Vector search via HelixDB HNSW index
- Score computed as `1 / (1 + distance)`
