# Design

**Crate:** `ix-cli`\
**Purpose:** Command-line interface for Ixchel

`ix-cli` is a thin binary wrapper over `ix-core` and a configured index backend.

## Architecture

```
ixchel (bin)
  ├─ ix-core (repo + markdown + validation)
  └─ ix-storage-helixdb (IndexBackend impl; rebuildable cache)
```

## Command Model

- Repo-level commands: `init`, `check`, `sync`
- Entity CRUD: `create`, `show`, `list`, `delete`, `edit`
- Relationships: `link`, `unlink`, `graph`, `context`
- Search: `search`

## JSON Output

`--json` switches output to JSON for AI/agent consumption. The CLI prints one
JSON object per invocation on stdout.

## Editor Launch

`ixchel edit <id>` launches `$IXCHEL_EDITOR` or `$EDITOR`, defaulting to `vi`.
