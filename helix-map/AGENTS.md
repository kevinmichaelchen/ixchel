# HELIX-MAP AGENTS

**Parent:** See `../AGENTS.md` for workspace context.

## Overview

Codebase indexer that builds a compact symbol skeleton for LLM context and fast
search. Current implementation targets Rust and stores JSON indexes.

## Structure

```
helix-map/
├── src/
│   ├── main.rs            # CLI entrypoint
│   ├── lib.rs             # Library exports
│   ├── scanner.rs         # File discovery
│   ├── extract.rs         # Tree-sitter extraction
│   ├── model.rs           # Symbol data types
│   ├── indexer.rs         # Index orchestration
│   ├── storage.rs         # JSON persistence
│   └── skeleton.rs        # Skeleton rendering
├── specs/
│   ├── requirements.md
│   ├── design.md
│   └── tasks.md
└── README.md
```

## Where To Look

| Task                    | Location                    |
| ----------------------- | --------------------------- |
| CLI behavior            | `helix-map/src/main.rs`     |
| Rust parsing/extraction | `helix-map/src/extract.rs`  |
| Index models            | `helix-map/src/model.rs`    |
| Index storage           | `helix-map/src/storage.rs`  |
| Skeleton output         | `helix-map/src/skeleton.rs` |

## Commands

```bash
cargo run -p helix-map -- index .
cargo run -p helix-map -- skeleton . --output -
```
