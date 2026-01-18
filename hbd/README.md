# hbd - Git-First Issue Tracker

A local-first issue tracker that stores issues as Markdown files in your repository. Designed for AI-assisted development workflows where issues need to be readable by both humans and AI agents.

The name pays homage to both **H**elix and **B**ea**d**s.

## Quick Start

```bash
# Initialize in your project
cd your-project
hbd init

# Create issues
hbd create "Fix login bug" --type bug --priority 1
hbd create "Add dark mode" --type feature --labels ui,frontend

# View and manage
hbd list                          # List all open issues
hbd show bd-a1b2c3                 # View issue details
hbd update bd-a1b2 --status in_progress
hbd close bd-a1b2 --reason "Fixed in commit abc123"

# Track dependencies
hbd dep add bd-blocker blocks bd-blocked
hbd ready                         # Issues with no open blockers
hbd blocked                       # Issues waiting on others
hbd explain bd-a1b2               # Show blocker tree

# Labels and comments
hbd label add bd-a1b2 urgent
hbd comment bd-a1b2 "Started investigating"
```

## Features

### Currently Implemented

- **Issue CRUD** - Create, show, list, update, close, reopen
- **Markdown storage** - Issues stored as `.tickets/*.md` with YAML frontmatter
- **Dependencies** - Track blocking relationships with cycle detection
- **Labels** - Add, remove, list labels on issues
- **Comments** - Add comments with human/agent attribution
- **Ready/Blocked** - Find unblocked issues ready for work
- **Stale detection** - Find issues not updated in N days
- **Statistics** - Issue counts by status, type, priority
- **Agent tracking** - `--agent` and `--session` flags for AI workflows
- **JSON output** - `--json` flag on all commands for programmatic access
- **Partial ID matching** - Use `bd-a1b` instead of full `bd-a1b2c3`

### Planned (Not Yet Implemented)

- **HelixDB integration** - Embedded graph database for fast queries (no server needed—like SQLite)
- **Semantic search** - Find issues by meaning, not just keywords (via local embeddings)
- **Hybrid search** - BM25 + vector fusion with reranking
- **Critical path analysis** - Find longest blocking chain to an epic
- **Sync daemon** - Background file watching and auto-sync
- **AI compaction** - Summarize old closed issues to save context

## Installation

```bash
# From source
git clone https://github.com/kevinmichaelchen/helix-tools.git
cd helix-tools
cargo install --path hbd
```

## File Format

Issues are stored as Markdown files with YAML frontmatter:

```markdown
---
id: bd-a1b2c3
title: Fix memory leak in parser
status: open
priority: 1
issue_type: bug
created_at: 2026-01-03T10:30:00Z
created_by: kevin
labels:
  - performance
  - parser
depends_on:
  - id: bd-x7y8z9
    dep_type: blocks
---

## Description

The parser leaks memory when processing large files...

## Comments

### 2026-01-03 14:22 - kevin

Started investigating, appears to be in the tokenizer.
```

## Project Structure

```
your-project/
├── .tickets/
│   ├── bd-a1b2c3.md    # Issue files
│   ├── bd-d4e5f6.md
│   └── ...
├── .helix/
│   └── config.toml     # hbd configuration
└── .gitignore          # .helix/helix.db/ auto-added
```

## CLI Reference

### Project Setup

| Command    | Description                         |
| ---------- | ----------------------------------- |
| `hbd init` | Initialize hbd in current directory |
| `hbd info` | Show project status and statistics  |

### Issue Management

| Command              | Description              |
| -------------------- | ------------------------ |
| `hbd create "Title"` | Create new issue         |
| `hbd show <id>`      | Display issue details    |
| `hbd list`           | List issues (filterable) |
| `hbd update <id>`    | Modify issue properties  |
| `hbd close <id>`     | Close an issue           |
| `hbd reopen <id>`    | Reopen a closed issue    |

### Dependencies

| Command                         | Description                |
| ------------------------------- | -------------------------- |
| `hbd dep add <a> blocks <b>`    | A blocks B                 |
| `hbd dep remove <a> blocks <b>` | Remove dependency          |
| `hbd dep list <id>`             | Show issue dependencies    |
| `hbd dep cycles`                | Find circular dependencies |
| `hbd ready`                     | List unblocked issues      |
| `hbd blocked`                   | List blocked issues        |
| `hbd explain <id>`              | Show blocker tree          |

### Labels & Comments

| Command                         | Description             |
| ------------------------------- | ----------------------- |
| `hbd label add <id> <label>`    | Add label to issue      |
| `hbd label remove <id> <label>` | Remove label            |
| `hbd label list <id>`           | List labels on issue    |
| `hbd label list-all`            | List all project labels |
| `hbd comment <id> "msg"`        | Add comment             |
| `hbd comments <id>`             | List comments           |

### Analytics

| Command               | Description       |
| --------------------- | ----------------- |
| `hbd stats`           | Issue statistics  |
| `hbd stale --days 14` | Find stale issues |

### Common Flags

| Flag             | Description           |
| ---------------- | --------------------- |
| `--json`         | Output as JSON        |
| `--agent <id>`   | Mark as agent-created |
| `--session <id>` | Group agent actions   |

## Why hbd?

### Standing on the Shoulders of Giants

hbd builds on excellent prior art in the git-backed issue tracking space:

**[Beads](https://github.com/steveyegge/beads)** (Steve Yegge) pioneered the vision of git-backed, AI-native issue tracking. Beads introduced hash-based IDs for conflict-free merging, dependency graphs with blocking semantics, AI compaction for context management, and agent coordination via gates. If you're happy with Beads, keep using it—it's battle-tested and feature-rich.

**[wedow/ticket](https://github.com/wedow/ticket)** took a radically minimal approach: a single shell script, no database, just Markdown files with YAML frontmatter in `.tickets/`. No daemon, no SQLite sync headaches. We adopted this Markdown-first storage approach directly.

### What hbd Adds

hbd exists because we wanted capabilities that file-based storage alone can't efficiently provide:

| Capability                 | Beads      | ticket | hbd        |
| -------------------------- | ---------- | ------ | ---------- |
| Git-backed storage         | ✅         | ✅     | ✅         |
| Markdown files             | ❌ (JSONL) | ✅     | ✅         |
| No daemon required         | ❌         | ✅     | ✅         |
| Dependency tracking        | ✅         | ✅     | ✅         |
| Cycle detection            | ✅         | ❌     | ✅         |
| **Semantic search**        | ❌         | ❌     | 🚧 Planned |
| **Graph algorithms**       | ❌         | ❌     | 🚧 Planned |
| **Critical path analysis** | ❌         | ❌     | 🚧 Planned |

**Semantic Search** — Find issues by _meaning_, not just keywords. Search for "user can't log in" and find issues about "authentication", "sign-in", and "login failures" even if they use different words. Get duplicate warnings when creating issues that are semantically similar to existing ones.

**Graph Algorithms** — Beads tracks dependencies; hbd _analyzes_ them. Find the critical path blocking your epic. Compute weighted paths based on priority × estimated time. Answer "where should I focus for maximum impact?"

**Native Graph Storage** — HelixDB is an embedded database (like SQLite—no server to run) purpose-built for graph + vector workloads. Instead of recursive SQL CTEs for transitive dependencies, we get single-hop traversals. Instead of application-level cycle detection, we get native BFS.

### What We Kept, What We Skipped

**From Beads, we kept:**

- Hash-based IDs (`bd-a1b2c3`) for conflict-free merging
- Dependency tracking with blocking semantics
- Agent tracking with `--agent` and `--session` flags
- `ready` and `blocked` commands
- Full offline support

**From Beads, we skipped:**

- Molecular chemistry (templates, wisps, bonds) — simpler epics + labels instead
- 12+ issue types — we have 5: bug, feature, task, epic, chore
- Background daemon — direct file access instead
- SQLite — Markdown files are the source of truth

### When to Use What

| Use case                                       | Recommendation             |
| ---------------------------------------------- | -------------------------- |
| Want mature, battle-tested                     | **Beads**                  |
| Want absolute minimalism (single shell script) | **wedow/ticket**           |
| Want semantic search + graph algorithms        | **hbd** (when implemented) |
| Already using HelixDB                          | **hbd**                    |
| Need molecular templates                       | **Beads**                  |

## Specifications

For detailed requirements, architecture, and implementation plans:

- [specs/requirements.md](./specs/requirements.md) - User stories and acceptance criteria
- [specs/design.md](./specs/design.md) - Technical architecture and data model
- [specs/tasks.md](./specs/tasks.md) - Implementation roadmap

## License

MIT
