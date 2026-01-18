# Entity Type Specifications

This document defines the six core entity types in the Helix knowledge graph.

## Common Properties

All entities share these base properties:

```yaml
# Required
id: string          # Unique identifier with type prefix (e.g., dec-a1b2c3)
title: string       # Human-readable title
created_at: datetime
updated_at: datetime

# Optional
uuid: string        # Hash-based UUID for rename safety
created_by: string  # Author name/handle
created_by_type: human | agent
agent_id: string    # If created by agent
session_id: string  # Agent session grouping
tags: string[]      # Arbitrary labels
external_ref: string # Link to external system (GitHub issue, etc.)
```

## Common Relationships

All entities can have:

```yaml
relates_to: id[]    # General association with other entities
```

---

## 1. Decision (`dec-`)

Architecture Decision Records (ADRs) — immutable after acceptance.

### Status Lifecycle

```
proposed → accepted → [superseded | deprecated]
    ↓
 rejected
```

### Properties

```yaml
# Decision-specific required
status: proposed | accepted | rejected | superseded | deprecated
date: date          # Decision date (not created_at)

# Decision-specific optional
deciders: string[]  # Who made this decision
context: string     # Why this decision was needed
consequences: string[] # Expected outcomes
alternatives: string[] # Options considered

# Relationships
supersedes: id[]    # Replaces these decisions
amends: id[]        # Modifies these decisions
depends_on: id[]    # Requires these decisions
spawns: id[]        # Issues/work created by this decision
cites: id[]         # Sources referenced
addresses: id[]     # Issues/ideas this responds to
```

### File Format

```markdown
---
id: dec-a1b2c3
title: Use PostgreSQL for Primary Storage
status: accepted
date: 2026-01-15
created_at: 2026-01-10T14:30:00Z
updated_at: 2026-01-15T09:00:00Z
created_by: kevin
deciders: [kevin, alice, bob]
tags: [database, infrastructure]
depends_on: [dec-8f9e0d]
cites: [src-pg2024]
addresses: [idea-7x8y9z]
---

## Context

We need a primary database for the application...

## Decision

We will use PostgreSQL 16 with...

## Consequences

### Positive

- ACID compliance...

### Negative

- Operational complexity...

## Alternatives Considered

### MySQL

Rejected because...

### MongoDB

Rejected because...
```

### Directory

```
.helix/decisions/
├── dec-a1b2c3.md
├── dec-d4e5f6.md
└── ...
```

### Immutability Rules

Once `status: accepted`:

- `title` cannot change
- `decision` section cannot change
- `status` can only change to `superseded` or `deprecated`
- Pre-commit hook enforces this

---

## 2. Issue (`iss-`)

Work items: bugs, features, tasks, epics, chores.

### Status Lifecycle

```
open → in_progress → [closed | blocked]
                          ↑
                     in_progress
```

### Properties

```yaml
# Issue-specific required
status: open | in_progress | blocked | closed

# Issue-specific optional
type: bug | feature | task | epic | chore
priority: 0-4       # 0=critical, 4=backlog
assignee: string
parent_id: id       # Parent epic/task
estimated_minutes: number
closed_at: datetime
closed_reason: string

# Relationships
blocks: id[]        # Issues this blocks
depends_on: id[]    # Issues blocking this
implements: id[]    # Decisions this implements
spawned_by: id      # Decision that created this
```

### File Format

```markdown
---
id: iss-b2c3d4
title: Implement Connection Pooling
status: open
type: feature
priority: 1
created_at: 2026-01-16T10:00:00Z
updated_at: 2026-01-16T10:00:00Z
created_by: kevin
assignee: alice
tags: [database, performance]
implements: [dec-a1b2c3]
spawned_by: dec-a1b2c3
depends_on: [iss-x1y2z3]
estimated_minutes: 480
---

## Description

Implement PgBouncer-style connection pooling...

## Acceptance Criteria

- [ ] Pool size configurable
- [ ] Connection timeout handling
- [ ] Metrics exposed

## Comments

### 2026-01-16T14:00:00Z — alice (human)

Started investigating pool libraries...
```

### Directory

```
.helix/issues/
├── iss-b2c3d4.md
├── iss-e5f6g7.md
└── ...
```

---

## 3. Idea (`idea-`)

Proposals, brainstorms, explorations — lightweight pre-decision artifacts.

### Status Lifecycle

```
draft → [proposed | parked | rejected]
            ↓
    evolved (became decision/issue)
```

### Properties

```yaml
# Idea-specific required
status: draft | proposed | parked | rejected | evolved

# Idea-specific optional
champion: string    # Who's advocating for this
effort: low | medium | high | unknown
impact: low | medium | high | unknown

# Relationships
evolves_into: id    # Decision or Issue this became
inspired_by: id[]   # Other ideas/sources that sparked this
```

### File Format

```markdown
---
id: idea-c3d4e5
title: What If We Used WebAssembly for Plugins?
status: draft
created_at: 2026-01-14T09:00:00Z
updated_at: 2026-01-14T09:00:00Z
created_by: bob
champion: bob
effort: high
impact: high
tags: [architecture, plugins, wasm]
inspired_by: [src-wasm-component]
---

## The Idea

Instead of native plugins, what if we compiled to WASM...

## Why This Might Work

- Sandboxing for free
- Language agnostic
- ...

## Open Questions

- Performance overhead?
- Debugging story?
- ...

## Next Steps

- [ ] Prototype with wasmtime
- [ ] Benchmark against native
```

### Directory

```
.helix/ideas/
├── idea-c3d4e5.md
└── ...
```

---

## 4. Report (`rpt-`)

Analysis documents: postmortems, RFCs, retrospectives, research findings.

### Status Lifecycle

```
draft → [published | archived]
```

### Properties

```yaml
# Report-specific required
status: draft | published | archived
report_type: postmortem | rfc | retrospective | analysis | research

# Report-specific optional
period_start: date  # For retrospectives
period_end: date
incident_date: datetime  # For postmortems

# Relationships
summarizes: id[]    # Issues/decisions this covers
cites: id[]         # Sources referenced
recommends: id[]    # Decisions/ideas proposed
addresses: id[]     # Issues this responds to
```

### File Format

```markdown
---
id: rpt-d4e5f6
title: Q4 2025 Database Performance Retrospective
status: published
report_type: retrospective
period_start: 2025-10-01
period_end: 2025-12-31
created_at: 2026-01-05T10:00:00Z
updated_at: 2026-01-08T16:00:00Z
created_by: alice
tags: [database, performance, quarterly]
summarizes: [iss-a1, iss-a2, iss-a3, dec-42]
cites: [src-pgtuning, src-pooling101]
recommends: [idea-c3d4e5]
---

## Executive Summary

Database performance improved 40% in Q4...

## Key Metrics

| Metric      | Q3    | Q4   | Change |
| ----------- | ----- | ---- | ------ |
| p99 latency | 120ms | 72ms | -40%   |

## What Went Well

1. Connection pooling (dec-42)...

## What Didn't Go Well

1. Migration rollback...

## Action Items

- [ ] Implement read replicas (→ idea-c3d4e5)
```

### Directory

```
.helix/reports/
├── rpt-d4e5f6.md
└── ...
```

---

## 5. Source (`src-`)

External references: papers, articles, documentation, books, talks.

### Properties

```yaml
# Source-specific required
source_type: paper | article | documentation | book | talk | video | repository

# Source-specific optional
url: string         # Original location
authors: string[]
published_date: date
publisher: string   # Journal, blog, conference
doi: string         # For papers
isbn: string        # For books
archived_at: string # Archive.org link
local_path: string  # Local PDF/file

# Relationships
cited_by: id[]      # Computed: entities citing this
```

### File Format

```markdown
---
id: src-e5f6g7
title: "Redis: An In-Memory Data Structure Store"
source_type: paper
url: https://example.com/redis-paper.pdf
authors: [Salvatore Sanfilippo]
published_date: 2019-06-30
publisher: SIGMOD 2019
doi: 10.1145/1234567.1234568
created_at: 2026-01-10T08:00:00Z
updated_at: 2026-01-10T08:00:00Z
created_by: kevin
tags: [redis, caching, database]
archived_at: https://web.archive.org/web/...
---

## Summary

This paper describes the design and implementation of Redis...

## Key Insights

1. Single-threaded event loop avoids locking overhead
2. RDB + AOF hybrid persistence strategy
3. ...

## Relevance to Our Work

We're considering Redis for session caching (see dec-42)...
```

### Directory

```
.helix/sources/
├── src-e5f6g7.md
└── ...
```

---

## 6. Citation (`cite-`)

Specific quotes, excerpts, or references from sources — granular attribution.

### Properties

```yaml
# Citation-specific required
from_source: id     # The source being cited
quote: string       # The actual text (or paraphrase)

# Citation-specific optional
page: string        # Page number or section
timestamp: string   # For videos/talks (e.g., "14:32")
is_paraphrase: boolean

# Relationships
supports: id[]      # Decisions/ideas this evidence supports
contradicts: id[]   # Decisions/ideas this evidence contradicts
used_in: id[]       # Reports/decisions where this is cited
```

### File Format

```markdown
---
id: cite-f6g7h8
title: Redis Single-Threaded Design Rationale
from_source: src-e5f6g7
page: "Section 3.2"
is_paraphrase: false
created_at: 2026-01-10T08:30:00Z
updated_at: 2026-01-10T08:30:00Z
created_by: kevin
tags: [redis, architecture]
supports: [dec-42]
used_in: [rpt-d4e5f6]
---

## Quote

> "The single-threaded nature of Redis is not a limitation but a feature.
> By avoiding locks entirely, Redis achieves predictable latency and
> simpler reasoning about data consistency."

## Context

The authors are responding to criticism about Redis's threading model...

## My Notes

This directly supports our decision to use Redis for session state,
where predictable latency matters more than raw throughput.
```

### Directory

```
.helix/citations/
├── cite-f6g7h8.md
└── ...
```

---

## ID Generation

IDs are generated using BLAKE3 hash of:

- Entity type prefix
- Title (normalized)
- Creation timestamp
- Random salt

Format: `{prefix}-{6-char-hex}`

Examples:

- `dec-a1b2c3`
- `iss-d4e5f6`
- `idea-g7h8i9`

Partial matching supported: `dec-a1` resolves to `dec-a1b2c3` if unambiguous.

---

## Embedding Strategy

All entities are embedded for semantic search. The embedding input is:

```
{title}

{body_text}

Tags: {tags_joined}
```

This ensures:

- Title matches score highly
- Body content is searchable
- Tags influence similarity

Embedding model: `BAAI/bge-small-en-v1.5` (384 dimensions) by default.

---

## Directory Structure

```
project/
├── .helix/
│   ├── config.toml          # Helix configuration
│   ├── data/                 # HelixDB storage (gitignored)
│   │   └── helix.db/
│   ├── decisions/
│   │   ├── dec-a1b2c3.md
│   │   └── ...
│   ├── issues/
│   │   ├── iss-d4e5f6.md
│   │   └── ...
│   ├── ideas/
│   │   └── ...
│   ├── reports/
│   │   └── ...
│   ├── sources/
│   │   └── ...
│   └── citations/
│       └── ...
└── .gitignore               # Includes .helix/data/
```
