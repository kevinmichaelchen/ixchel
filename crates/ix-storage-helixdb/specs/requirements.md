# Requirements

This document defines requirements for `ix-storage-helixdb` (HelixDB backend for Ixchel).

## 1. Index Backend Contract

### US-001: Sync rebuild

| ID       | Acceptance Criterion                                                                        |
| -------- | ------------------------------------------------------------------------------------------- |
| AC-001.1 | THE SYSTEM SHALL implement `ix_core::index::IndexBackend` for HelixDB                       |
| AC-001.2 | WHEN `sync(repo)` is called THE SYSTEM SHALL rebuild the local cache from `.ixchel/**/*.md` |
| AC-001.3 | THE SYSTEM SHALL store rebuildable data under `.ixchel/data/` (configurable subpath)        |
| AC-001.4 | THE SYSTEM SHALL embed entity text and store vectors for semantic search                    |
| AC-001.5 | THE SYSTEM SHALL store entity nodes and relationship edges                                  |

### US-002: Search

| ID       | Acceptance Criterion                                                               |
| -------- | ---------------------------------------------------------------------------------- |
| AC-002.1 | WHEN `search(query, limit)` is called THE SYSTEM SHALL return ranked semantic hits |
| AC-002.2 | THE SYSTEM SHALL return `id`, `title`, optional `kind`, and a normalized `score`   |

### US-003: Health

| ID       | Acceptance Criterion                                                        |
| -------- | --------------------------------------------------------------------------- |
| AC-003.1 | WHEN `health_check()` is called THE SYSTEM SHALL verify storage readability |

## 2. Embedding

### US-004: Local embedding provider

| ID       | Acceptance Criterion                                                          |
| -------- | ----------------------------------------------------------------------------- |
| AC-004.1 | THE SYSTEM SHALL support `fastembed` as an embedding provider                 |
| AC-004.2 | WHERE the configured provider is unsupported THE SYSTEM SHALL return an error |
