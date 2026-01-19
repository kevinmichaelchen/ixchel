# Design

**Document:** design.md\
**Status:** Active (2026-01-06)\
**Author:** Kevin Chen

This document describes the design decisions and implementation details for `ix-config`.

## Overview

`ix-config` provides hierarchical configuration loading for Ixchel. It implements a unified directory structure with layered config from global, project, and environment sources.

## Design Goals

1. **Unified Home** - Single `~/.ixchel/` directory for global config/state/data
2. **Consistency** - All tools use the same config loading pattern
3. **Predictability** - Clear precedence rules for config merging
4. **Convenience** - Auto-detection of common settings (GitHub token)
5. **Simplicity** - Minimal API surface, easy to use correctly

## Directory Structure

### Unified ~/.ixchel/ Home

```
~/.ixchel/                        # Ixchel root directory
├── config/                       # Configuration files (user-editable)
│   ├── config.toml               # Shared settings
│   └── ixchel.toml               # ixchel-specific
│
├── data/                         # Caches & databases (auto-generated)
│
├── state/                        # Runtime metadata (ephemeral)
│   ├── agents/                   # Agent registry
│   ├── locks/                    # Process locks
│   └── cache/                    # Computed caches
│
├── log/                          # Operation logs
│
└── .ixchel-version               # Metadata
```

### Project Local

```
{project}/.ixchel/                 # Project config only (no data)
├── config.toml                   # Project shared config
└── ixchel.toml                   # ixchel project overrides
```

### Why Unified?

**Old model (problematic):**

```
~/.config/<legacy>/  ← configs
~/.cache/<legacy>/   ← data
```

**New model (unified):**

```
~/.ixchel/            ← everything
├── config/          ← configs (user edits)
├── data/            ← caches (auto-generated)
├── state/           ← metadata (ephemeral)
└── log/             ← debugging
```

Benefits:

- Single location (easy to manage, backup, reset)
- Clear separation (config vs data vs state)
- Follows Rust/Node/Ruby tool patterns
- Clear what's safe to delete

---

## Configuration Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│                  Environment Variables                   │  Priority 1 (highest)
│          IXCHEL_HOME, IXCHEL_GITHUB__TOKEN, etc.          │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                  Global Tool Config                      │  Priority 2
│              ~/.ixchel/config/<tool>.toml                 │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                 Global Shared Config                     │  Priority 3
│               ~/.ixchel/config/config.toml                │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                  Project Tool Config                     │  Priority 4
│                  .ixchel/<tool>.toml                      │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                 Project Shared Config                    │  Priority 5
│                   .ixchel/config.toml                     │
└─────────────────────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────┐
│                      Defaults                            │  Priority 6 (lowest)
│                  (from Default trait)                    │
└─────────────────────────────────────────────────────────┘
```

---

## Shared Config Schema

The shared `config.toml` contains settings used by multiple tools:

```toml
[github]
token = "ghp_xxx"  # Most tools need GitHub access

[embedding]
provider = "fastembed"
model = "BAAI/bge-small-en-v1.5"
batch_size = 32
dimension = 384

[storage]
backend = "helixdb"
path = "data/ixchel" # relative to .ixchel/
```

### SharedConfig Struct

```rust
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IxchelConfig {
    #[serde(default)]
    pub github: GitHubConfig,
    #[serde(default)]
    pub embedding: EmbeddingConfig,
    #[serde(default)]
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitHubConfig {
    pub token: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingConfig {
    #[serde(default = "default_embedding_provider")]
    pub provider: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
    #[serde(default)]
    pub dimension: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    #[serde(default = "default_storage_backend")]
    pub backend: String,
    #[serde(default = "default_storage_path")]
    pub path: String,
}
```

---

## API Design

### Path Helpers

```rust
/// Get the Ixchel home directory (~/.ixchel or $IXCHEL_HOME)
pub fn ixchel_home() -> PathBuf {
    if let Ok(home) = std::env::var("IXCHEL_HOME") {
        return PathBuf::from(home);
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".ixchel")
}

/// Get the config directory (~/.ixchel/config)
pub fn ixchel_config_dir() -> PathBuf {
    ixchel_home().join("config")
}

/// Get the data directory (~/.ixchel/data)
pub fn ixchel_data_dir() -> PathBuf {
    ixchel_home().join("data")
}

/// Get the state directory (~/.ixchel/state)
pub fn ixchel_state_dir() -> PathBuf {
    ixchel_home().join("state")
}

/// Get the log directory (~/.ixchel/log)
pub fn ixchel_log_dir() -> PathBuf {
    ixchel_home().join("log")
}

/// Find project config directory (.ixchel/) by walking up from cwd
pub fn find_project_config_dir() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    let root = find_git_root(&cwd)?;
    let ixchel_dir = root.join(".ixchel");
    ixchel_dir.exists().then_some(ixchel_dir)
}
```

### Config Loading

```rust
/// Simple config loading with defaults
pub fn load_config<T: DeserializeOwned + Default>(tool_name: &str) -> Result<T, ConfigError> {
    ConfigLoader::new(tool_name).load()
}

/// Load shared config (github token, embedding model, etc.)
pub fn load_shared_config() -> Result<SharedConfig, ConfigError> {
    ConfigLoader::new("").load_shared()
}
```

### Builder API

```rust
pub struct ConfigLoader {
    tool_name: String,
    env_prefix: Option<String>,
    project_dir: Option<PathBuf>,
    global_dir: Option<PathBuf>,
}

impl ConfigLoader {
    pub fn new(tool_name: impl Into<String>) -> Self;
    
    /// Set custom environment variable prefix (default: HELIX_{TOOL})
    pub fn with_env_prefix(self, prefix: impl Into<String>) -> Self;
    
    /// Override project config directory
    pub fn with_project_dir(self, path: impl Into<PathBuf>) -> Self;
    
    /// Override global config directory
    pub fn with_global_dir(self, path: impl Into<PathBuf>) -> Self;
    
    /// Load and merge config from all sources
    pub fn load<T: DeserializeOwned + Default>(self) -> Result<T, ConfigError>;
    
    /// Load shared config only
    pub fn load_shared(self) -> Result<SharedConfig, ConfigError>;
}
```

### GitHub Token Helper

```rust
/// Detect GitHub token from multiple sources
pub fn detect_github_token() -> Option<String> {
    // 1. Environment variables (highest priority)
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        return Some(token);
    }
    if let Ok(token) = std::env::var("GH_TOKEN") {
        return Some(token);
    }
    
    // 2. Config file
    if let Ok(config) = load_shared_config() {
        if let Some(token) = config.github.token {
            return Some(token);
        }
    }
    
    // 3. gh CLI fallback
    std::process::Command::new("gh")
        .args(["auth", "token"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
}
```

---

## Merge Strategy

### Scalar Values

Later values replace earlier values.

### Tables (Objects)

Merged recursively. Keys in higher-priority configs override lower-priority.

### Arrays

Replaced entirely (not concatenated). This prevents unexpected behavior.

### Example

```toml
# ~/.ixchel/config/config.toml (global)
[github]
token = "global_token"

[embedding]
provider = "fastembed"
model = "small-model"
batch_size = 16
dimension = 384

# .ixchel/config.toml (project)
[embedding]
batch_size = 32  # Override just this field

# Result:
# github.token = "global_token"
# embedding.provider = "fastembed"
# embedding.model = "small-model"
# embedding.batch_size = 32
# embedding.dimension = 384
```

---

## Error Types

```rust
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file {}: {source}", path.display())]
    ReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse config file {}: {source}", path.display())]
    ParseError {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("Failed to write config file {}: {source}", path.display())]
    WriteError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to serialize config file {}: {source}", path.display())]
    SerializeError {
        path: PathBuf,
        #[source]
        source: toml::ser::Error,
    },
}
```

---

## Security Considerations

### Token Handling

- Never log token values
- Never include tokens in error messages
- Consider `secrecy` crate for memory safety

### File Permissions

- Warn if config files are world-readable
- Recommend `chmod 600` for files containing secrets

---

## Implementation Notes

### Loading Order

```rust
pub fn load<T: DeserializeOwned + Default>(self) -> Result<T, ConfigError> {
    let mut merged = toml::Table::new();

    // 1. Project shared config: .ixchel/config.toml
    let project_dir = self.project_dir.or_else(find_project_config_dir);
    if let Some(dir) = &project_dir {
        if let Some(table) = load_toml_file(&dir.join("config.toml"))? {
            merge_tables(&mut merged, table);
        }
    }

    // 2. Project tool config: .ixchel/<tool>.toml
    if let Some(dir) = project_dir {
        let tool_config = dir.join(format!("{}.toml", self.tool_name));
        if let Some(table) = load_toml_file(&tool_config)? {
            merge_tables(&mut merged, table);
        }
    }

    // 3. Global shared config: ~/.ixchel/config/config.toml
    let global_dir = self.global_dir.unwrap_or_else(ixchel_config_dir);
    if let Some(table) = load_toml_file(&global_dir.join("config.toml"))? {
        merge_tables(&mut merged, table);
    }

    // 4. Global tool config: ~/.ixchel/config/<tool>.toml
    let tool_config = global_dir.join(format!("{}.toml", self.tool_name));
    if let Some(table) = load_toml_file(&tool_config)? {
        merge_tables(&mut merged, table);
    }

    // 5. Environment variables (TODO: implement env overlay)

    // 6. Default if empty
    if merged.is_empty() {
        return Ok(T::default());
    }

    // Deserialize merged config
    let value = toml::Value::Table(merged);
    value.try_into().map_err(|e| ConfigError::ParseError {
        path: PathBuf::from("<merged>"),
        source: e,
    })
}
```

---

## Consumers

| Tool / Crate | Tool Name | Config Path                    | Primary Data Location |
| ------------ | --------- | ------------------------------ | --------------------- |
| `ix-cli`     | `ixchel`  | `~/.ixchel/config/ixchel.toml` | `{repo}/.ixchel/`     |
| `ix-mcp`     | `ixchel`  | `~/.ixchel/config/ixchel.toml` | `{repo}/.ixchel/`     |

---

## Migration Guide

### From Legacy Directory Structure

Old:

```
~/.config/<legacy>/config.toml
~/.cache/<legacy>/
```

New:

```
~/.ixchel/config/config.toml
~/.ixchel/data/
```

Migration steps:

1. Create `~/.ixchel/` directory
2. Move legacy config files into `~/.ixchel/config/`
3. Move legacy cache files into `~/.ixchel/data/`
4. Remove old directories

### For Tool Developers

1. Replace custom config loading with `ix_config::load_config("tool-name")`
2. Use `ix_config::ixchel_data_dir().join("tool-data")` for data storage
3. Use `ix_config::detect_github_token()` for GitHub access

---

## Storage Path Conventions

> **Note:** ix-config does NOT depend on HelixDB. It only documents path conventions that tools should follow when they use HelixDB as their storage backend.
>
> See [ADR-004](../../../.ixchel/decisions/legacy/004-trait-based-storage-architecture.md) for why storage is each tool's concern.

### Project-Local Storage (Default)

For tools that store data within the project:

```
{project}/.ixchel/data/{tool}/
├── data.mdb                 # LMDB data file (if using HelixDB)
└── lock.mdb                 # LMDB lock file
```

Tools may store rebuildable caches under `.ixchel/data/{tool}/` when project-local storage is appropriate.

### Global Storage

For tools with cross-project data:

```
~/.ixchel/data/{tool}/
├── data.mdb
└── lock.mdb
```

**Example:** Ixchel may store rebuildable caches in `~/.ixchel/data/`

### Path Helper Functions

```rust
use ix_config::{ixchel_data_dir, project_data_dir};

// Global data path
let global_data = ixchel_data_dir().join("docs");
// → ~/.ixchel/data/docs/

// Project-local data path (requires git root discovery)
let project_data = project_data_dir("my-tool")?;
// → {project}/.ixchel/data/my-tool/
```

### Storage Config Options

Tools can configure storage behavior via shared config:

```toml
# ~/.ixchel/config/config.toml

[storage]
# LMDB map size (default: 1GB for project-local, 10GB for global)
map_size_mb = 1024

# Max readers (default: 126)
max_readers = 126
```

Tools read this config and pass it to their storage backend implementation.

---

## See Also

- [requirements.md](./requirements.md) — Requirements specification
- [ADR-004](../../../.ixchel/decisions/legacy/004-trait-based-storage-architecture.md) — Trait-based storage architecture
