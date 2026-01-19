# ix-storage-helixdb

HelixDB-backed storage adapter for **Ixchel**.

Implements the `ix_core::index::IndexBackend` contract so apps and domain logic
don’t depend directly on HelixDB.

By default the HelixDB instance is stored under `.ixchel/data/ixchel/` and is
safe to delete/rebuild via `ixchel sync`.

## Kiro Specs

- [Requirements][requirements]
- [Design][design]
- [Tasks][tasks]

<!-- Links -->

[requirements]: specs/requirements.md
[design]: specs/design.md
[tasks]: specs/tasks.md
