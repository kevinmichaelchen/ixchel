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
