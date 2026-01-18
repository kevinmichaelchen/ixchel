# ix-cli

Command-line interface for **Ixchel**.

`ixchel` is intended to be a thin wrapper around `ix-core`.

## Quick Start

```bash
# from a git repo
ixchel init

ixchel create decision "Use PostgreSQL for primary storage"
ixchel create issue "Implement connection pooling"
ixchel link iss-xxxx implements dec-xxxx

ixchel sync
ixchel search "database performance" --limit 10
```

Ixchel stores canonical knowledge artifacts as Markdown under `.ixchel/`. The
graph/vector index lives under `.ixchel/data/` and is rebuildable.
