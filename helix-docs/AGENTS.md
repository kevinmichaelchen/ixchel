# HELIX-DOCS AGENTS

**Parent:** See `../AGENTS.md` for workspace context.

## Overview

Global documentation cache with semantic search. CLI + library organized into
ports (traits) and services.

## Structure

```
helix-docs/
├── src/
│   ├── main.rs            # CLI entrypoint
│   ├── lib.rs             # Library exports
│   ├── cli/               # CLI subcommands
│   ├── domain/            # Core types (Source/Document/Chunk)
│   ├── ports/             # Traits (fetch, embed, repo, search)
│   ├── services/          # Ingestion/search services
│   ├── config/            # Config loading
│   └── error.rs
├── specs/                 # requirements/design/tasks
└── README.md
```

## Where To Look

| Task                   | Location                   |
| ---------------------- | -------------------------- |
| CLI commands           | `helix-docs/src/cli/`      |
| Domain models          | `helix-docs/src/domain/`   |
| Port traits            | `helix-docs/src/ports/`    |
| Ingestion/search logic | `helix-docs/src/services/` |
| Config handling        | `helix-docs/src/config/`   |

## Commands

```bash
cargo run -p helix-docs -- --help
```
