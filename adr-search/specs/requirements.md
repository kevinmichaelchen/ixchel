# adr-search: Requirements Specification

**Document:** requirements.md  
**Status:** Concept (2026-01-05)  
**Author:** Kevin Chen

## User Stories

### For Agents
```
As an AI agent building context for a task,
I want to search for relevant architecture decisions quickly,
So that I can understand the design constraints and avoid conflicts.
```

**Acceptance Criteria:**
- `adr-search "database migration" --json` returns ranked results in < 100ms
- Output includes: id, title, status, score, tags, date, deciders
- Semantic search (not just keyword matching)
- Works offline (no external API calls)

### For Developers
```
As a developer,
I want to quickly find past architectural decisions from the terminal,
So that I can reference the reasoning behind design choices.
```

**Acceptance Criteria:**
- `adr-search "caching"` returns human-readable results
- Can filter by status: `adr-search "caching" --status accepted`
- First run indexes all ADRs (~2-5s), subsequent runs are fast
- Works in any repo with `.decisions/` directory

### For CI/Automation
```
As a CI pipeline,
I want to verify that relevant decisions exist before deployment,
So that deployments align with architectural decisions.
```

**Acceptance Criteria:**
- `adr-search "deployment strategy" --limit 1` exits 0 if found, 1 if not
- Works with shell pipelines

## Functional Requirements

### FR-1: Load ADRs from Filesystem
- **Input:** Directory path (default: `.decisions/`)
- **Output:** Parsed ADR objects with metadata
- **Requirements:**
  - Support markdown files: `NNN-title-kebab-case.md`
  - Parse YAML frontmatter (id, title, status, date, deciders, tags)
  - Extract body text for embedding
  - Handle missing/malformed files gracefully

### FR-2: Index into HelixDB
- **Input:** Parsed ADRs
- **Output:** Persistent vector index in `~/.helix/data/adr/`
- **Requirements:**
  - Embed each ADR body using fastembed (CPU)
  - Store embeddings + metadata in HelixDB
  - Create index on first run (~2-5s for 100 ADRs)
  - HelixDB persists across invocations

### FR-3: Delta Indexing
- **Input:** Current ADR directory
- **Output:** Updated HelixDB index
- **Requirements:**
  - Track file hashes to detect changes
  - Re-index only changed/new ADRs
  - Remove deleted ADRs from index
  - Execute in < 100ms for typical case (no changes)

### FR-4: Semantic Search
- **Input:** Query string
- **Output:** Ranked ADRs with scores (0.0-1.0)
- **Requirements:**
  - Embed query using same model as ADRs
  - Search HelixDB for nearest neighbors
  - Return top-K results sorted by score
  - Execute in < 50ms

### FR-5: Metadata Filtering
- **Input:** Status and/or tags
- **Output:** Filtered ranked results
- **Requirements:**
  - Filter by status: `--status accepted`
  - Filter by tags: `--tags database,api`
  - Combine filters with AND logic

### FR-6: Output Formatting
- **Input:** Search results
- **Output:** Formatted text (pretty or JSON)
- **Requirements:**
  - Pretty: Human-readable with colors
  - JSON: Machine-readable with full metadata
  - Default: Pretty if terminal, JSON if piped

### FR-7: CLI Interface
- **Requirements:**
  - Command: `adr-search <QUERY> [OPTIONS]`
  - Options: `--directory`, `--limit`, `--status`, `--tags`, `--json`
  - Help: `adr-search --help`

## Non-Functional Requirements

### Performance
- First search: 2-5 seconds (indexing)
- Subsequent search: < 100ms
- Query embedding: 50-100ms
- HelixDB search: < 50ms

### Reliability
- Handle missing `.decisions/` gracefully
- Handle malformed YAML (skip with warning)
- Handle deleted ADRs (remove from index)

### Compatibility
- Linux, macOS, Windows
- Works offline
- Embedded HelixDB (no separate setup)

## Data Model

### ADR Metadata (Required)
```yaml
id: integer
title: string
status: enum  # proposed | accepted | superseded | deprecated
date: date    # YYYY-MM-DD
deciders: [string]
tags: [string]
```

### SearchResult
```json
{
  "id": 3,
  "title": "Database Migration Strategy",
  "status": "accepted",
  "score": 0.89,
  "tags": ["database", "migration"],
  "date": "2026-01-04",
  "deciders": ["Alice", "Bob"],
  "file_path": ".decisions/003-database-migration-strategy.md"
}
```

## CLI Specification

```
adr-search <QUERY> [OPTIONS]

ARGS:
  <QUERY>  Search query

OPTIONS:
  -d, --directory <PATH>   ADR directory (default: .decisions/)
  -l, --limit <N>          Max results (default: 10)
  --status <STATUS>        Filter by status
  --tags <TAGS>            Filter by tags (comma-separated)
  -j, --json               JSON output
  -h, --help               Show help
  -V, --version            Show version
```

### Exit Codes
- 0: Success (results found)
- 1: No results found
- 2: Error

## Out of Scope

- ADR creation
- ADR editing
- Format validation
- Git operations
- Approval workflows
