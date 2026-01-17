# HELIX-REPO AGENTS

**Parent:** See `../AGENTS.md` for workspace context.

## Overview

Repository clone manager. Provides both a CLI and a library API for cloning to a
standardized directory layout. Implementation is scaffolded per specs.

## Structure

```
helix-repo/
├── src/
│   ├── main.rs            # CLI entrypoint
│   ├── lib.rs             # Library API
│   └── error.rs           # Error types
├── specs/                 # requirements/design/tasks
└── README.md
```

## Where To Look

| Task                     | Location                  |
| ------------------------ | ------------------------- |
| CLI wiring               | `helix-repo/src/main.rs`  |
| Repository manager API   | `helix-repo/src/lib.rs`   |
| Error handling           | `helix-repo/src/error.rs` |
| Roadmap and requirements | `helix-repo/specs/`       |

## Commands

```bash
cargo run -p helix-repo -- --help
```
