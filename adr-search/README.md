# adr-search

Semantic search over Architecture Decision Records, backed by embedded HelixDB.

**Status:** Scaffolded  
**Created:** 2026-01-05

## Why adr-search?

Agents need quick access to architectural decisions for context:
- "What have we decided about caching?"
- "Why did we choose X over Y?"
- "Are we already committed to this direction?"

But `.decisions/` directories are unindexed markdown. adr-search makes them **searchable in < 100ms** via semantic indexing.

## Core Idea

```bash
# First invocation: index all ADRs into HelixDB
adr-search "database migration" 

# Result: Ranked ADRs with scores and metadata
# [
#   { id: 3, title: "...", status: "accepted", score: 0.87, ... },
#   { id: 1, title: "...", status: "proposed", score: 0.72, ... }
# ]

# Second invocation: fast delta + search
adr-search "caching strategy"  # < 100ms (mostly search, minimal indexing)
```

**Key:** Persistent HelixDB index, delta indexing on each call.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# Search ADRs (default: .decisions/)
adr-search <QUERY> [OPTIONS]

# Options:
--directory <PATH>         # ADR directory (default: .decisions/)
--limit <N>                # Results limit (default: 10)
--status <STATUS>          # Filter by status (proposed|accepted|superseded|deprecated)
--tags <TAGS>              # Filter by tags (comma-separated)
--json                     # JSON output (default for piping)

# Examples:
adr-search "caching"
adr-search --directory ./architecture "database" --status accepted
adr-search "performance" --limit 3 --json
```

## Output

### Pretty (human-readable)
```
[1] ADR-003: Database Migration Strategy
    Status: accepted
    Score: 0.89
    Tags: database, migration, infrastructure

[2] ADR-001: Schema Versioning Approach
    Status: proposed
    Score: 0.71
    Tags: database, schema, testing
```

### JSON (machine-readable)
```json
{
  "query": "database migration",
  "results": [
    {
      "id": 3,
      "title": "Database Migration Strategy",
      "status": "accepted",
      "score": 0.89,
      "tags": ["database", "migration"],
      "file_path": ".decisions/003-database-migration-strategy.md"
    }
  ]
}
```

## ADR Format

ADRs should be markdown files with YAML frontmatter:

```yaml
---
id: 3
title: Database Migration Strategy
status: accepted
date: 2026-01-04
deciders:
  - Alice
  - Bob
tags:
  - database
  - migration
---

# Context and Problem Statement
...

# Decision
...
```

## How It Works

1. **First invocation:** Scan `.decisions/`, embed with fastembed, store in HelixDB (~2-5s)
2. **Subsequent invocations:** Delta check (file hashes), re-index only changed ADRs, search (~100ms)

## Architecture

```
User/Agent
    │
    ↓
┌─────────────────────────────────┐
│   adr-search CLI                │
└──────────┬──────────────────────┘
           │
    ┌──────▼──────────────────────┐
    │ Embedded HelixDB             │
    │ • Vector index               │
    │ • Persistent (~/.helix/)     │
    └──────────────────────────────┘
```

## Specs

See `specs/` directory:
- [requirements.md](specs/requirements.md) - User stories, acceptance criteria
- [design.md](specs/design.md) - Architecture, data model
- [tasks.md](specs/tasks.md) - Implementation phases

## License

MIT
