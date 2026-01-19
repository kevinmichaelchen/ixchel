# ix-mcp

MCP server for **Ixchel**.

This crate should expose a stable, agent-friendly tool surface and delegate the
actual behavior to `ix-core` (domain) and `ix-app` (wiring).

## Tools

- `ixchel_sync` — rebuild `.ixchel/data/` from Markdown
- `ixchel_search` — semantic search over entities
- `ixchel_show` — read an entity by id
- `ixchel_graph` — list outgoing relationships
- `ixchel_context` — assemble a basic 1-hop context pack

## Kiro Specs

- [Requirements][requirements]
- [Design][design]
- [Tasks][tasks]

<!-- Links -->

[requirements]: specs/requirements.md
[design]: specs/design.md
[tasks]: specs/tasks.md
