# Requirements

This document defines requirements for `ix-cli` (Ixchel CLI).

## 1. CLI Surface

### US-001: Initialize repository

| ID       | Acceptance Criterion                                                              |
| -------- | --------------------------------------------------------------------------------- |
| AC-001.1 | WHEN a user runs `ixchel init` THE SYSTEM SHALL create `.ixchel/` in the git root |
| AC-001.2 | WHERE `--force` is provided THE SYSTEM SHALL recreate missing layout + config     |

### US-002: Manage entities

| ID       | Acceptance Criterion                                                                                         |
| -------- | ------------------------------------------------------------------------------------------------------------ |
| AC-002.1 | WHEN a user runs `ixchel create <kind> <title>` THE SYSTEM SHALL create a Markdown entity                    |
| AC-002.2 | WHEN a user runs `ixchel list [kind]` THE SYSTEM SHALL list entities                                         |
| AC-002.3 | WHEN a user runs `ixchel show <id>` THE SYSTEM SHALL print raw Markdown for that id                          |
| AC-002.4 | WHEN a user runs `ixchel delete <id>` THE SYSTEM SHALL delete the entity file                                |
| AC-002.5 | WHEN a user runs `ixchel edit <id>` THE SYSTEM SHALL open the entity in `$IXCHEL_EDITOR`/`$EDITOR`           |
| AC-002.6 | WHERE `--sort recent` is provided (or default) THE SYSTEM SHALL sort list results by `created_at` descending |
| AC-002.7 | WHERE `--sort updated` is provided THE SYSTEM SHALL sort list results by `updated_at` descending             |

### US-003: Manage relationships

| ID       | Acceptance Criterion                                                                     |
| -------- | ---------------------------------------------------------------------------------------- |
| AC-003.1 | WHEN a user runs `ixchel link <from> <rel> <to>` THE SYSTEM SHALL add the relationship   |
| AC-003.2 | WHEN a user runs `ixchel unlink <from> <rel> <to>` THE SYSTEM SHALL remove it if present |
| AC-003.3 | WHEN a user runs `ixchel graph <id>` THE SYSTEM SHALL print outgoing relationships       |
| AC-003.4 | WHEN a user runs `ixchel context <id>` THE SYSTEM SHALL print a 1-hop context pack       |

### US-004: Validate repo

| ID       | Acceptance Criterion                                                           |
| -------- | ------------------------------------------------------------------------------ |
| AC-004.1 | WHEN a user runs `ixchel check` THE SYSTEM SHALL validate entity ids and links |
| AC-004.2 | IF validation fails THEN THE SYSTEM SHALL exit non-zero                        |

## 2. Search & Sync

### US-005: Build rebuildable cache

| ID       | Acceptance Criterion                                                                               |
| -------- | -------------------------------------------------------------------------------------------------- |
| AC-005.1 | WHEN a user runs `ixchel sync` THE SYSTEM SHALL rebuild `.ixchel/data/` via the configured backend |

### US-006: Semantic search

| ID       | Acceptance Criterion                                                         |
| -------- | ---------------------------------------------------------------------------- |
| AC-006.1 | WHEN a user runs `ixchel search <query>` THE SYSTEM SHALL return ranked hits |
| AC-006.2 | WHERE `--limit` is provided THE SYSTEM SHALL cap results                     |

## 3. Machine-Readable Output

### US-007: JSON output

| ID       | Acceptance Criterion                                                            |
| -------- | ------------------------------------------------------------------------------- |
| AC-007.1 | WHERE `--json` is provided THE SYSTEM SHALL print stable-ish JSON payloads      |
| AC-007.2 | IF JSON output is enabled and `check` fails THEN THE SYSTEM SHALL exit non-zero |

## 4. Metadata Discovery

### US-008: List tags

| ID       | Acceptance Criterion                                                                                 |
| -------- | ---------------------------------------------------------------------------------------------------- |
| AC-008.1 | WHEN a user runs `ixchel tags` THE SYSTEM SHALL list all unique tags with usage counts               |
| AC-008.2 | WHERE `--kind <kind>` is provided THE SYSTEM SHALL list tags from that kind only                     |
| AC-008.3 | WHERE `--untagged` is provided THE SYSTEM SHALL list entities missing tags                           |
| AC-008.4 | THE SYSTEM SHALL treat tags as case-sensitive, trimmed strings and ignore empty values               |
| AC-008.5 | THE SYSTEM SHALL count each tag at most once per entity                                              |
| AC-008.6 | THE SYSTEM SHALL sort tags alphabetically                                                            |
| AC-008.7 | WHERE `--json` is provided THE SYSTEM SHALL output a JSON object with `total` and `tags`             |
| AC-008.8 | WHERE `--untagged --json` is provided THE SYSTEM SHALL output a JSON object with `total` and `items` |

### US-009: Modify tags

| ID       | Acceptance Criterion                                                                          |
| -------- | --------------------------------------------------------------------------------------------- |
| AC-009.1 | WHEN a user runs `ixchel tag add <id> <tag>...` THE SYSTEM SHALL add those tags to the entity |
| AC-009.2 | WHEN a user runs `ixchel tag remove <id> <tag>...` THE SYSTEM SHALL remove those tags         |
| AC-009.3 | THE SYSTEM SHALL treat tag add/remove operations as idempotent                                |
| AC-009.4 | WHERE `--json` is provided THE SYSTEM SHALL output `id`, `action`, `changed`, and `tags`      |
