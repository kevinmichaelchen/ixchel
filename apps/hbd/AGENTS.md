# HBD AGENTS

**Parent:** See `../../AGENTS.md` for workspace context.

## Overview

Git-first issue tracker. Issues are stored as Markdown with YAML frontmatter in
`.ixchel/issues/` and manipulated via the `hbd` CLI.

## Structure

```
apps/hbd/
├── src/
│   ├── main.rs            # CLI entrypoint
│   ├── lib.rs             # Library exports
│   ├── commands/          # CLI subcommand handlers
│   ├── storage.rs         # TicketStore file I/O
│   ├── markdown.rs        # Frontmatter serialize/parse
│   ├── types.rs           # Issue/Status/Priority/etc.
│   ├── domain/            # Graph + filters
│   ├── id.rs              # ID generation helpers
│   └── db.rs              # HelixDB placeholders (todo)
├── specs/                 # requirements/design/tasks
└── README.md              # CLI reference + format
```

## Where To Look

| Task                   | Location                                             |
| ---------------------- | ---------------------------------------------------- |
| Add a CLI command      | `apps/hbd/src/main.rs` + `apps/hbd/src/commands/`    |
| Modify issue schema    | `apps/hbd/src/types.rs` + `apps/hbd/src/markdown.rs` |
| Ticket file I/O        | `apps/hbd/src/storage.rs`                            |
| Dependency graph logic | `apps/hbd/src/domain/graph.rs`                       |
| ID format              | `apps/hbd/src/id.rs`                                 |
| HelixDB integration    | `apps/hbd/src/db.rs` (currently todo)                |

## Commands

```bash
cargo run -p hbd -- --help
cargo run -p hbd -- init
cargo run -p hbd -- create "Title" --type bug
```
