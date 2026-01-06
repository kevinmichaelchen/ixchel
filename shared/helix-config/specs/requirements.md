# Requirements

This document defines requirements for `helix-config` using [EARS notation](https://www.iaria.org/conferences2015/filesICCGI15/EARS.pdf).

## EARS Notation Reference

| Pattern | Template |
|---------|----------|
| Ubiquitous | THE SYSTEM SHALL `<action>` |
| Event-driven | WHEN `<trigger>` THE SYSTEM SHALL `<action>` |
| State-driven | WHILE `<state>` THE SYSTEM SHALL `<action>` |
| Optional | WHERE `<feature>` THE SYSTEM SHALL `<action>` |
| Complex | IF `<condition>` THEN THE SYSTEM SHALL `<action>` |

---

## 1. Directory Structure

### US-001: Helix Home Directory

**As a** developer  
**I want to** access a unified helix home directory  
**So that** all helix data is organized in one place

| ID | Acceptance Criterion |
|----|---------------------|
| AC-001.1 | THE SYSTEM SHALL use `~/.helix/` as the default helix home directory |
| AC-001.2 | THE SYSTEM SHALL respect `HELIX_HOME` environment variable if set |
| AC-001.3 | THE SYSTEM SHALL provide `helix_home()` function returning the home path |
| AC-001.4 | THE SYSTEM SHALL create the home directory if it does not exist (when writing) |

---

### US-002: Helix Subdirectories

**As a** developer  
**I want to** access specific helix subdirectories  
**So that** I can organize config, data, state, and logs

| ID | Acceptance Criterion |
|----|---------------------|
| AC-002.1 | THE SYSTEM SHALL provide `helix_config_dir()` returning `~/.helix/config/` |
| AC-002.2 | THE SYSTEM SHALL provide `helix_data_dir()` returning `~/.helix/data/` |
| AC-002.3 | THE SYSTEM SHALL provide `helix_state_dir()` returning `~/.helix/state/` |
| AC-002.4 | THE SYSTEM SHALL provide `helix_log_dir()` returning `~/.helix/log/` |

---

## 2. Configuration Loading

### US-003: Load Tool Configuration

**As a** developer  
**I want to** load configuration for my helix tool  
**So that** I can access user settings consistently

| ID | Acceptance Criterion |
|----|---------------------|
| AC-003.1 | WHEN `load_config(tool_name)` is called THE SYSTEM SHALL return a deserialized config struct |
| AC-003.2 | THE SYSTEM SHALL merge configs from global, project, and environment sources |
| AC-003.3 | IF no config files exist THEN THE SYSTEM SHALL return default values |
| AC-003.4 | IF config parsing fails THEN THE SYSTEM SHALL return a descriptive error |

---

### US-004: Global Configuration

**As a** developer  
**I want to** store global settings  
**So that** they apply across all projects

| ID | Acceptance Criterion |
|----|---------------------|
| AC-004.1 | THE SYSTEM SHALL read global shared config from `~/.helix/config/config.toml` |
| AC-004.2 | THE SYSTEM SHALL read tool-specific global config from `~/.helix/config/<tool>.toml` |
| AC-004.3 | IF the config directory does not exist THEN THE SYSTEM SHALL skip global config |

---

### US-005: Project Configuration

**As a** developer  
**I want to** store project-specific settings  
**So that** each project can customize behavior

| ID | Acceptance Criterion |
|----|---------------------|
| AC-005.1 | THE SYSTEM SHALL read project shared config from `.helix/config.toml` |
| AC-005.2 | THE SYSTEM SHALL read tool-specific project config from `.helix/<tool>.toml` |
| AC-005.3 | THE SYSTEM SHALL search for `.helix/` in the current working directory |
| AC-005.4 | IF no `.helix/` directory exists THEN THE SYSTEM SHALL skip project config |

---

### US-006: Configuration Merging

**As a** developer  
**I want to** config values to merge predictably  
**So that** I can override global settings per-project

| ID | Acceptance Criterion |
|----|---------------------|
| AC-006.1 | THE SYSTEM SHALL merge configs with project values overriding global values |
| AC-006.2 | THE SYSTEM SHALL merge nested tables recursively |
| AC-006.3 | THE SYSTEM SHALL replace arrays entirely (not append) |
| AC-006.4 | THE SYSTEM SHALL treat missing keys as "use parent value" |

---

### US-007: Environment Variable Overrides

**As a** developer  
**I want to** override config via environment variables  
**So that** I can configure tools in CI/CD and containers

| ID | Acceptance Criterion |
|----|---------------------|
| AC-007.1 | WHEN `HELIX_HOME` is set THE SYSTEM SHALL use that as the helix home directory |
| AC-007.2 | WHEN `HELIX_{SETTING}` is set THE SYSTEM SHALL use that for the corresponding setting |
| AC-007.3 | THE SYSTEM SHALL convert `__` to nested keys (e.g., `HELIX_GITHUB__TOKEN` → `github.token`) |
| AC-007.4 | Environment variables SHALL take highest priority |

---

## 3. Shared Settings

### US-008: GitHub Token Detection

**As a** developer  
**I want to** GitHub tokens to be auto-detected  
**So that** I don't have to configure them manually

| ID | Acceptance Criterion |
|----|---------------------|
| AC-008.1 | THE SYSTEM SHALL check `GITHUB_TOKEN` environment variable first |
| AC-008.2 | THE SYSTEM SHALL check `GH_TOKEN` environment variable second |
| AC-008.3 | THE SYSTEM SHALL check `github.token` in config files third |
| AC-008.4 | THE SYSTEM SHALL run `gh auth token` as fallback if gh CLI is available |
| AC-008.5 | THE SYSTEM SHALL return `None` if no token is found |
| AC-008.6 | THE SYSTEM SHALL never log or display the token value |

---

### US-009: Shared Config Schema

**As a** developer  
**I want to** access shared settings  
**So that** I can use GitHub tokens and embedding models consistently

| ID | Acceptance Criterion |
|----|---------------------|
| AC-009.1 | THE SYSTEM SHALL provide `SharedConfig` struct with common fields |
| AC-009.2 | THE SYSTEM SHALL include `github.token` in shared config |
| AC-009.3 | THE SYSTEM SHALL include `embedding.model` in shared config |
| AC-009.4 | THE SYSTEM SHALL include `embedding.batch_size` in shared config |
| AC-009.5 | THE SYSTEM SHALL include `storage.base` in shared config (default: `~/.helix/data`) |

---

## 4. Project Root Discovery

### US-010: Find Project Root

**As a** developer  
**I want to** `.helix/` to be found relative to working directory  
**So that** project config works from subdirectories

| ID | Acceptance Criterion |
|----|---------------------|
| AC-010.1 | THE SYSTEM SHALL search for `.helix/` in the current working directory |
| AC-010.2 | WHERE `walk_up` is enabled THE SYSTEM SHALL search parent directories |
| AC-010.3 | THE SYSTEM SHALL stop at filesystem root |
| AC-010.4 | THE SYSTEM SHALL provide `find_project_root()` function |

---

## Non-Functional Requirements

### NFR-001: Performance

| ID | Requirement |
|----|-------------|
| NFR-001.1 | Config loading SHALL complete in under 10 milliseconds |
| NFR-001.2 | Path resolution SHALL be lazy (not computed until needed) |

### NFR-002: Security

| ID | Requirement |
|----|-------------|
| NFR-002.1 | THE SYSTEM SHALL never log sensitive values (tokens, passwords) |
| NFR-002.2 | THE SYSTEM SHALL warn if config file permissions are too open (world-readable) |

### NFR-003: Dependencies

| ID | Requirement |
|----|-------------|
| NFR-003.1 | THE SYSTEM SHALL have minimal dependencies (serde, toml, dirs) |
| NFR-003.2 | THE SYSTEM SHALL not require async runtime |

### NFR-004: Portability

| ID | Requirement |
|----|-------------|
| NFR-004.1 | THE SYSTEM SHALL work on macOS, Linux, and Windows |
| NFR-004.2 | THE SYSTEM SHALL expand `~` in paths on all platforms |
