# Requirements

This document defines requirements for `helix-repo` using [EARS notation](https://www.iaria.org/conferences2015/filesICCGI15/EARS.pdf).

## EARS Notation Reference

| Pattern      | Template                                          |
| ------------ | ------------------------------------------------- |
| Ubiquitous   | THE SYSTEM SHALL `<action>`                       |
| Event-driven | WHEN `<trigger>` THE SYSTEM SHALL `<action>`      |
| State-driven | WHILE `<state>` THE SYSTEM SHALL `<action>`       |
| Optional     | WHERE `<feature>` THE SYSTEM SHALL `<action>`     |
| Complex      | IF `<condition>` THEN THE SYSTEM SHALL `<action>` |

---

## 1. Repository Cloning

### US-001: Clone Repository

**As a** developer or tool\
**I want to** clone a git repository to a standard location\
**So that** I have a local copy organized by domain/owner/repo

| ID       | Acceptance Criterion                                                                                   |
| -------- | ------------------------------------------------------------------------------------------------------ |
| AC-001.1 | WHEN `clone <url>` is called THE SYSTEM SHALL clone the repository to `{root}/{domain}/{owner}/{repo}` |
| AC-001.2 | THE SYSTEM SHALL support HTTPS URLs (e.g., `https://github.com/facebook/react`)                        |
| AC-001.3 | THE SYSTEM SHALL support SSH URLs (e.g., `git@github.com:facebook/react.git`)                          |
| AC-001.4 | THE SYSTEM SHALL support schemeless URLs (e.g., `github.com/facebook/react`)                           |
| AC-001.5 | THE SYSTEM SHALL strip `.git` extension from the destination path                                      |
| AC-001.6 | IF the repository already exists THEN THE SYSTEM SHALL return the existing path without re-cloning     |
| AC-001.7 | THE SYSTEM SHALL create parent directories as needed                                                   |

---

### US-002: Clone Options

**As a** developer\
**I want to** customize how repositories are cloned\
**So that** I can optimize for my use case

| ID       | Acceptance Criterion                                                                  |
| -------- | ------------------------------------------------------------------------------------- |
| AC-002.1 | WHERE `--shallow` is specified THE SYSTEM SHALL perform a shallow clone (depth 1)     |
| AC-002.2 | WHERE `--branch <name>` is specified THE SYSTEM SHALL clone the specified branch      |
| AC-002.3 | WHERE `--dry-run` is specified THE SYSTEM SHALL print the operation without executing |
| AC-002.4 | WHERE `--json` is specified THE SYSTEM SHALL output results as JSON                   |

---

## 2. Repository Listing

### US-003: List Repositories

**As a** developer\
**I want to** list all cloned repositories\
**So that** I can see what's available locally

| ID       | Acceptance Criterion                                                                              |
| -------- | ------------------------------------------------------------------------------------------------- |
| AC-003.1 | WHEN `list` is called THE SYSTEM SHALL display all cloned repositories                            |
| AC-003.2 | THE SYSTEM SHALL show repository name and path for each entry                                     |
| AC-003.3 | WHERE `--filter <pattern>` is specified THE SYSTEM SHALL filter repositories matching the pattern |
| AC-003.4 | WHERE `--json` is specified THE SYSTEM SHALL output as JSON                                       |

---

### US-004: Repository Info

**As a** developer\
**I want to** get detailed information about a repository\
**So that** I can understand its state

| ID       | Acceptance Criterion                                                     |
| -------- | ------------------------------------------------------------------------ |
| AC-004.1 | WHEN `info <name>` is called THE SYSTEM SHALL display repository details |
| AC-004.2 | THE SYSTEM SHALL show: path, URL, current branch, last fetch time        |
| AC-004.3 | THE SYSTEM SHALL accept `owner/repo` format for the name                 |
| AC-004.4 | IF repository is not found THEN THE SYSTEM SHALL return an error         |

---

## 3. Repository Removal

### US-005: Remove Repository

**As a** developer\
**I want to** remove a cloned repository\
**So that** I can free disk space

| ID       | Acceptance Criterion                                                            |
| -------- | ------------------------------------------------------------------------------- |
| AC-005.1 | WHEN `remove <name>` is called THE SYSTEM SHALL delete the repository directory |
| AC-005.2 | THE SYSTEM SHALL remove empty parent directories after deletion                 |
| AC-005.3 | WHERE `--dry-run` is specified THE SYSTEM SHALL print what would be deleted     |
| AC-005.4 | IF repository is not found THEN THE SYSTEM SHALL return an error                |

---

## 4. Configuration

### US-006: Root Directory

**As a** developer\
**I want to** configure where repositories are cloned\
**So that** I can control disk usage

| ID       | Acceptance Criterion                                               |
| -------- | ------------------------------------------------------------------ |
| AC-006.1 | THE SYSTEM SHALL use `~/.cache/helix/repos` as the default root    |
| AC-006.2 | THE SYSTEM SHALL respect `HELIX_REPO_ROOT` environment variable    |
| AC-006.3 | THE SYSTEM SHALL read root from `~/.config/helix/helix-repo.toml`  |
| AC-006.4 | WHEN `root` is called THE SYSTEM SHALL print the current root path |
| AC-006.5 | Environment variable SHALL override config file value              |

---

## 5. URL Parsing

### US-007: URL Normalization

**As a** developer\
**I want to** use various URL formats\
**So that** I don't have to remember exact syntax

| ID       | Acceptance Criterion                                                |
| -------- | ------------------------------------------------------------------- |
| AC-007.1 | THE SYSTEM SHALL parse HTTPS URLs with or without `.git` suffix     |
| AC-007.2 | THE SYSTEM SHALL parse SSH URLs in `git@host:path` format           |
| AC-007.3 | THE SYSTEM SHALL parse SSH URLs in `ssh://git@host/path` format     |
| AC-007.4 | THE SYSTEM SHALL add `https://` to schemeless URLs containing a dot |
| AC-007.5 | IF URL is invalid THEN THE SYSTEM SHALL return a descriptive error  |

---

## 6. Library API

### US-008: Programmatic Access

**As a** helix-tools developer\
**I want to** use helix-repo as a library\
**So that** my tool can manage repositories

| ID       | Acceptance Criterion                                             |
| -------- | ---------------------------------------------------------------- |
| AC-008.1 | THE SYSTEM SHALL expose a `RepositoryManager` struct             |
| AC-008.2 | THE SYSTEM SHALL provide `clone(url) -> Result<PathBuf>`         |
| AC-008.3 | THE SYSTEM SHALL provide `find(name) -> Result<Option<PathBuf>>` |
| AC-008.4 | THE SYSTEM SHALL provide `list() -> Result<Vec<Repository>>`     |
| AC-008.5 | THE SYSTEM SHALL provide `remove(name) -> Result<()>`            |

---

## Non-Functional Requirements

### NFR-001: Performance

| ID        | Requirement                                                     |
| --------- | --------------------------------------------------------------- |
| NFR-001.1 | Repository listing SHALL complete in under 100ms for 1000 repos |
| NFR-001.2 | Path calculation SHALL be pure (no I/O)                         |

### NFR-002: Compatibility

| ID        | Requirement                                                   |
| --------- | ------------------------------------------------------------- |
| NFR-002.1 | THE SYSTEM SHALL work on macOS, Linux, and Windows            |
| NFR-002.2 | THE SYSTEM SHALL require only `git` as an external dependency |

### NFR-003: Error Handling

| ID        | Requirement                                                            |
| --------- | ---------------------------------------------------------------------- |
| NFR-003.1 | THE SYSTEM SHALL provide clear error messages with actionable guidance |
| NFR-003.2 | THE SYSTEM SHALL use distinct exit codes for different error types     |
