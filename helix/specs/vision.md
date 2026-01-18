# Helix: A Unified Knowledge Graph System

## Vision Statement

Helix is a **local-first, git-native knowledge operating system** that unifies disparate knowledge artifacts — decisions, issues, ideas, reports, sources, and citations — into a single, semantically-searchable graph database with a unified CLI and TUI.

## The Problem

Software teams accumulate knowledge in silos:

- **ADRs** in `/docs/decisions/` — forgotten after approval
- **Issues** in GitHub/Jira — disconnected from architecture
- **Ideas** in Notion/Slack — lost to the void
- **Reports** in Google Docs — never referenced again
- **Papers/Sources** in browser bookmarks — impossible to rediscover
- **Citations** scattered in comments — no traceability

These artifacts are **deeply interconnected** but stored in **incompatible systems**:

- A decision spawns issues
- An issue cites a source
- A report summarizes a postmortem
- An idea evolves into a decision

Yet we cannot:

1. Search across all knowledge types semantically
2. Traverse the relationships between artifacts
3. Answer "why did we build it this way?"
4. Generate context for AI assistants
5. Maintain knowledge hygiene (stale, orphaned, contradictory)

## The Solution

Helix provides:

### 1. Unified Entity Model

All knowledge artifacts share:

- Markdown body with YAML frontmatter
- Unique identifier with type prefix (`dec-`, `iss-`, `idea-`, etc.)
- Creation/modification timestamps
- Creator attribution (human or agent)
- Semantic embedding for search
- Relationship links to other entities

### 2. Graph-Vector Storage

HelixDB provides:

- **Graph layer**: Typed nodes and edges for relationship traversal
- **Vector layer**: HNSW index for semantic similarity search
- **Secondary indices**: Fast lookups by ID, status, tags, dates

### 3. Git-Native Persistence

- Source of truth: Markdown files in `.helix/` directories
- HelixDB is a cache/index, rebuildable from files
- Full version history via git
- Branch/merge workflow for knowledge
- Pre-commit hooks for integrity enforcement

### 4. Unified CLI

One command for all entity types:

```bash
helix create decision "Use PostgreSQL for primary storage"
helix create issue "Implement connection pooling" --relates-to dec-42
helix search "database performance" --types decision,issue,source
helix graph dec-42 --depth 3
helix context iss-17 --for-agent
```

### 5. Interactive TUI

- Entity browser with type filtering
- Graph visualization
- Semantic search interface
- Relationship editor
- Knowledge health dashboard

### 6. AI-Native Design

- `--agent` and `--session` flags for attribution
- Context generation for LLM consumption
- Semantic search tuned for AI retrieval
- Compaction/summarization for long-lived entities

## Design Principles

### 1. Files Are the API

Every entity is a Markdown file. You can:

- Edit with any text editor
- Grep with standard tools
- Diff across git commits
- Review in pull requests

HelixDB indexes files; it doesn't own them.

### 2. Relationships Are First-Class

Edges are as important as nodes:

- `blocks`, `depends_on`, `relates_to`
- `supersedes`, `amends`, `evolves_into`
- `cites`, `quotes`, `supports`, `contradicts`

Graph queries answer real questions:

- "What's blocking this issue?"
- "What decisions cite this paper?"
- "Show me the evolution of this idea"

### 3. Semantic Search Everywhere

Every entity gets embedded. Search by meaning:

- "memory optimization" finds issues about allocation, decisions about pooling, papers about GC
- "authentication" finds OAuth decisions, login bugs, security RFCs

### 4. Knowledge Has Lifecycle

Entities age, evolve, and die:

- Ideas → accepted/rejected
- Decisions → active/superseded/deprecated
- Issues → open/closed
- Sources → cited/forgotten

Helix tracks and surfaces this.

### 5. AI Is a Collaborator

Agents create knowledge too:

- Issues from automated analysis
- Summaries from compaction
- Suggestions from similarity

Attribution matters. Track who (or what) created each artifact.

## Entity Types

| Type     | Prefix  | Purpose                     | Example                            |
| -------- | ------- | --------------------------- | ---------------------------------- |
| Decision | `dec-`  | Architecture choices (ADRs) | "Use event sourcing for audit log" |
| Issue    | `iss-`  | Work items, bugs, tasks     | "Fix memory leak in parser"        |
| Idea     | `idea-` | Proposals, brainstorms      | "What if we used WebAssembly?"     |
| Report   | `rpt-`  | Analysis, postmortems, RFCs | "Q4 Performance Retrospective"     |
| Source   | `src-`  | External references         | "Redis SIGMOD 2019 Paper"          |
| Citation | `cite-` | Specific quotes/excerpts    | "Key insight about LSM trees"      |

## Relationship Types

| Relationship   | Meaning                   | Example                      |
| -------------- | ------------------------- | ---------------------------- |
| `blocks`       | A must complete before B  | Issue blocks Issue           |
| `depends_on`   | A requires B to exist     | Decision depends on Decision |
| `relates_to`   | General association       | Any → Any                    |
| `supersedes`   | A replaces B              | Decision supersedes Decision |
| `amends`       | A modifies B              | Decision amends Decision     |
| `evolves_into` | A became B                | Idea evolves into Decision   |
| `spawns`       | A created B               | Decision spawns Issue        |
| `cites`        | A references B            | Report cites Source          |
| `quotes`       | A excerpts B              | Citation quotes Source       |
| `supports`     | A provides evidence for B | Citation supports Decision   |
| `contradicts`  | A conflicts with B        | Source contradicts Source    |
| `summarizes`   | A condenses B             | Report summarizes Issues     |
| `addresses`    | A responds to B           | Decision addresses Issue     |

## Success Metrics

1. **Discoverability**: Find relevant knowledge in <1s semantic search
2. **Traceability**: Answer "why?" by traversing 3+ hops in the graph
3. **Freshness**: Surface stale/orphaned knowledge automatically
4. **Adoption**: Replace 3+ external tools with unified workflow
5. **AI Utility**: Generate useful context for LLM assistants

## Non-Goals

- **Cloud sync**: Git handles distribution
- **Real-time collaboration**: Async via git branches
- **WYSIWYG editing**: Markdown is the interface
- **Universal search**: Scoped to project repositories
- **Replacing GitHub Issues**: Complement, not replace

## Prior Art

- **Obsidian**: Graph-based note linking (but not git-native, not typed)
- **Notion**: Unified workspace (but cloud-only, not semantic search)
- **Roam Research**: Bidirectional links (but not structured entities)
- **Logseq**: Local-first outliner (but not graph-vector hybrid)
- **Steve Yegge's Beads**: Inspiration for hbd (but theoretical)
- **ADR Tools**: Decision records (but no search, no relationships)

Helix synthesizes the best of these with:

- Typed entities (not just pages)
- Graph + vector search (not just links)
- Git-native storage (not proprietary sync)
- CLI-first interface (not GUI-only)
- AI-native design (not bolted on)

## The Name

**Helix** evokes:

- **DNA double helix**: Intertwined strands of knowledge
- **Spiral growth**: Ideas evolving through iterations
- **Graph structure**: Nodes connected in complex patterns
- **The Helix editor**: Terminal-native, modern, Rust-based

The CLI is simply `helix`. Crates follow `helix-*` naming.
