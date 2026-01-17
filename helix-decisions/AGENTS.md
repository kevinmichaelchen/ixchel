# HELIX-DECISIONS AGENTS

**Parent:** See `../AGENTS.md` for workspace context.

## Overview

Decision graph CLI for `.decisions/` with semantic search and relationship
tracking. Uses HelixDB and local embeddings for indexing.

## Structure

```
helix-decisions/
├── src/
│   ├── main.rs            # CLI entrypoint
│   ├── lib.rs             # Library exports
│   ├── types.rs           # Decision models
│   ├── loader.rs          # Markdown/frontmatter loading
│   ├── searcher.rs        # Query execution
│   ├── embeddings.rs      # Embedding pipeline
│   ├── storage.rs         # Storage trait + adapters
│   ├── helix_backend.rs   # HelixDB implementation
│   ├── hooks.rs           # Git hooks
│   ├── config.rs          # Config loading
│   ├── manifest.rs        # Index metadata
│   └── delta.rs           # Change detection
├── specs/                 # requirements/design/tasks
├── docs/                  # Phase notes and corrections
└── README.md
```

## Where To Look

| Task                | Location                               |
| ------------------- | -------------------------------------- |
| CLI commands        | `helix-decisions/src/main.rs`          |
| Decision schema     | `helix-decisions/src/types.rs`         |
| Frontmatter parsing | `helix-decisions/src/loader.rs`        |
| Index/search logic  | `helix-decisions/src/searcher.rs`      |
| HelixDB backend     | `helix-decisions/src/helix_backend.rs` |
| Git hook behavior   | `helix-decisions/src/hooks.rs`         |

## Commands

```bash
cargo run -p helix-decisions -- --help
cargo run -p helix-decisions -- search "architecture"
```
