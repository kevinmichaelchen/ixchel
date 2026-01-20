# Design

**Crate:** `ix-mcp`\
**Purpose:** MCP tool server for Ixchel

## Overview

`ix-mcp` provides an agent-friendly tool surface over stdin/stdout using
JSON-RPC 2.0 following the MCP tool conventions.

This crate is intentionally thin:

- protocol parsing + dispatch lives here
- repository logic is delegated to `ix-core`
- indexing/search is delegated to an `IndexBackend` implementation

## Tool Implementation Notes

- Tools accept a `repo` argument (path) to support multi-repo usage.
- Outputs are returned as text content containing pretty-printed JSON.

## Tools

| Tool             | Description                                    |
| ---------------- | ---------------------------------------------- |
| `ixchel_sync`    | Rebuild local cache from `.ixchel/**/*.md`     |
| `ixchel_search`  | Semantic search over indexed entities          |
| `ixchel_show`    | Read an entity by id                           |
| `ixchel_graph`   | Inspect outgoing relationships for an entity   |
| `ixchel_context` | Assemble a 1-hop context pack around an entity |
| `ixchel_tags`    | List all tags with usage counts                |

## Tag Discovery for Agents

`ixchel_tags` returns all unique tags with counts. Agents can use this to discover
the existing vocabulary before creating entities. LLMs are smart enough to detect
similarity and synonyms themselves.

Arguments:

- `repo` (optional): Repository path
- `kind` (optional): Filter tags to a specific entity kind
- `untagged` (optional): Return entities missing tags instead of tag counts

Returns: JSON object with `total` and `tags` array of `{tag, count}` objects,
sorted alphabetically. When `untagged` is true, returns `items` with entity
summaries instead.
