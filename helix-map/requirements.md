# Requirements

This document defines user stories and acceptance criteria for `helix-map` using
[EARS notation][ears] (Easy Approach to Requirements Syntax).

[ears]: https://www.iaria.org/conferences2013/filesICCGI13/ICCGI_2013_Tutorial_Terzakis.pdf

## EARS Notation Reference

| Pattern      | Template                                          |
| ------------ | ------------------------------------------------- |
| Ubiquitous   | THE SYSTEM SHALL `<action>`                       |
| Event-driven | WHEN `<trigger>` THE SYSTEM SHALL `<action>`      |
| State-driven | WHILE `<state>` THE SYSTEM SHALL `<action>`       |
| Optional     | WHERE `<feature>` THE SYSTEM SHALL `<action>`     |
| Complex      | IF `<condition>` THEN THE SYSTEM SHALL `<action>` |

---

## 1. Repository Indexing

### US-001: Index a Repository

**As a** developer **I want to** index a repository into a symbol graph **So
that** tools can understand the codebase without full source

| ID       | Acceptance Criterion                                                                                   |
| -------- | ------------------------------------------------------------------------------------------------------ |
| AC-001.1 | WHEN a user indexes a repository THE SYSTEM SHALL scan source files and detect supported languages     |
| AC-001.2 | WHEN a supported file is parsed THE SYSTEM SHALL extract symbol signatures (functions, types, exports) |
| AC-001.3 | WHEN indexing completes THE SYSTEM SHALL store symbols and relationships in HelixDB                    |
| AC-001.4 | WHEN a file is not supported THE SYSTEM SHALL skip it and record a warning                             |

---

### US-002: Incremental Updates

**As a** developer **I want** indexing to be incremental **So that** updates are
fast after small changes

| ID       | Acceptance Criterion                                                        |
| -------- | --------------------------------------------------------------------------- |
| AC-002.1 | WHEN a file hash is unchanged THE SYSTEM SHALL skip re-parsing that file    |
| AC-002.2 | WHEN a file changes THE SYSTEM SHALL update only affected symbols and edges |
| AC-002.3 | THE SYSTEM SHALL remove symbols that no longer exist in the file            |

---

### US-003: Skeleton Output

**As a** developer **I want** a compact skeleton view **So that** LLM context
stays small but accurate

| ID       | Acceptance Criterion                                                                         |
| -------- | -------------------------------------------------------------------------------------------- |
| AC-003.1 | WHEN a skeleton is generated THE SYSTEM SHALL include public signatures and type definitions |
| AC-003.2 | WHEN a symbol has documentation THE SYSTEM SHALL include a one-line summary                  |
| AC-003.3 | WHERE a user requests private symbols THE SYSTEM SHALL include them in the skeleton          |

---

### US-004: Symbol Queries

**As a** developer **I want** to query symbols by name and kind **So that**
tools can navigate code quickly

| ID       | Acceptance Criterion                                                                      |
| -------- | ----------------------------------------------------------------------------------------- |
| AC-004.1 | WHEN a user searches by name THE SYSTEM SHALL return matching symbols with file locations |
| AC-004.2 | WHEN a user filters by kind THE SYSTEM SHALL restrict results to that kind                |
| AC-004.3 | WHEN a symbol is selected THE SYSTEM SHALL return its signature and related edges         |

---

### US-005: Language Support

**As a** developer **I want** multi-language parsing **So that** monorepos are
supported

| ID       | Acceptance Criterion                                                   |
| -------- | ---------------------------------------------------------------------- |
| AC-005.1 | THE SYSTEM SHALL support a language plug-in model for extractors       |
| AC-005.2 | WHEN a language plug-in is missing THE SYSTEM SHALL degrade gracefully |

---

### US-006: Export to LLM Context

**As an** AI agent **I want** a minimal context export **So that** the LLM can
operate with high awareness

| ID       | Acceptance Criterion                                                            |
| -------- | ------------------------------------------------------------------------------- |
| AC-006.1 | WHEN exporting context THE SYSTEM SHALL include module paths and public symbols |
| AC-006.2 | WHEN a token budget is provided THE SYSTEM SHALL truncate deterministically     |

---

## Non-Functional Requirements

### NFR-001: Performance

| ID        | Requirement                                                                     |
| --------- | ------------------------------------------------------------------------------- |
| NFR-001.1 | THE SYSTEM SHALL index a 100k LOC repo in under 60 seconds on a laptop          |
| NFR-001.2 | THE SYSTEM SHALL complete incremental re-index of a single file in under 250 ms |

### NFR-002: Reliability

| ID        | Requirement                                                             |
| --------- | ----------------------------------------------------------------------- |
| NFR-002.1 | THE SYSTEM SHALL avoid corrupting HelixDB on crash (use atomic updates) |
| NFR-002.2 | THE SYSTEM SHALL log parse failures with file and language info         |

### NFR-003: Usability

| ID        | Requirement                                                    |
| --------- | -------------------------------------------------------------- |
| NFR-003.1 | THE SYSTEM SHALL provide a human-readable skeleton export      |
| NFR-003.2 | THE SYSTEM SHALL provide a machine-readable export for tooling |
