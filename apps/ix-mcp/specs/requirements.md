# Requirements

This document defines requirements for `ix-mcp` (Ixchel MCP server).

## 1. Protocol

### US-001: JSON-RPC/MCP basics

| ID       | Acceptance Criterion                                                                 |
| -------- | ------------------------------------------------------------------------------------ |
| AC-001.1 | THE SYSTEM SHALL accept JSON-RPC 2.0 requests on stdin and write responses to stdout |
| AC-001.2 | THE SYSTEM SHALL implement `initialize`, `tools/list`, and `tools/call`              |
| AC-001.3 | THE SYSTEM SHALL ignore requests with missing ids (notifications)                    |

## 2. Tool Surface

### US-002: Core tools

| ID       | Acceptance Criterion                                                                        |
| -------- | ------------------------------------------------------------------------------------------- |
| AC-002.1 | THE SYSTEM SHALL expose `ixchel_sync` for rebuilding the local cache from `.ixchel/**/*.md` |
| AC-002.2 | THE SYSTEM SHALL expose `ixchel_search` for semantic search                                 |
| AC-002.3 | THE SYSTEM SHALL expose `ixchel_show` for reading an entity by id                           |
| AC-002.4 | THE SYSTEM SHALL expose `ixchel_graph` for outgoing relationship inspection                 |
| AC-002.5 | THE SYSTEM SHALL expose `ixchel_context` for assembling a basic 1-hop context pack          |

### US-003: Repo targeting

| ID       | Acceptance Criterion                                                              |
| -------- | --------------------------------------------------------------------------------- |
| AC-003.1 | WHERE `arguments.repo` is provided THE SYSTEM SHALL operate relative to that path |
| AC-003.2 | IF `arguments.repo` is missing THE SYSTEM SHALL default to process CWD            |

### US-004: Tag discovery tool

| ID       | Acceptance Criterion                                                         |
| -------- | ---------------------------------------------------------------------------- |
| AC-004.1 | THE SYSTEM SHALL expose `ixchel_tags` for listing all tags with usage counts |
| AC-004.2 | THE SYSTEM SHALL return a JSON object with `total` and `tags` array          |
| AC-004.3 | THE SYSTEM SHALL sort tags alphabetically                                   |
