# helix-config

Hierarchical configuration loading for helix-tools. Provides a consistent config loading pattern across all CLI tools in the ecosystem.

## Why

All helix-tools share common configuration patterns:
- Global config in `~/.helix/config/`
- Project config in `.helix/`
- Environment variable overrides
- Shared settings (GitHub token, embedding model, etc.)

This crate centralizes that logic to avoid divergence.

## Directory Structure

helix-tools uses a unified `~/.helix/` directory for everything:

```
~/.helix/                         # Unified helix home
├── config/                       # Configuration (user-editable TOML)
│   ├── config.toml               # Shared settings (GitHub token, embedding model)
│   ├── hbd.toml                  # hbd settings
│   ├── helix-docs.toml           # helix-docs settings
│   ├── helix-map.toml            # helix-map settings
│   ├── helix-repo.toml           # helix-repo settings
│   └── helix-mail.toml           # helix-mail settings
│
├── data/                         # Caches & databases (auto-generated)
│   ├── docs/                     # helix-docs cache
│   └── index/                    # helix-map index
│
├── state/                        # Runtime metadata (ephemeral)
└── log/                          # Operation logs
```

Project-local overrides:

```
{project}/.helix/                 # Project config only (no data)
├── config.toml                   # Project shared config
├── hbd.toml                      # hbd project overrides
├── helix-docs.toml               # helix-docs project overrides
└── ...
```

See the [Configuration docs][config-docs] for full details.

## Configuration Hierarchy

```
Priority (highest to lowest):
1. Environment variables (HELIX_*, GITHUB_TOKEN, etc.)
2. Project tool config (.helix/<tool>.toml)
3. Project shared config (.helix/config.toml)
4. Global tool config (~/.helix/config/<tool>.toml)
5. Global shared config (~/.helix/config/config.toml)
6. Defaults (from Default trait)
```

## Usage

```rust
use helix_config::{ConfigLoader, load_config};
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct MyToolConfig {
    #[serde(default)]
    concurrency: usize,
}

// Simple loading with defaults
let config: MyToolConfig = load_config("my-tool")?;

// Or with more control
let loader = ConfigLoader::new("my-tool");
let config: MyToolConfig = loader
    .with_env_prefix("MY_TOOL")
    .load()?;
```

## Shared Config

The global `~/.helix/config/config.toml` contains settings shared across tools:

```toml
[github]
token = "ghp_xxx"  # Or use GITHUB_TOKEN env var

[embedding]
model = "BAAI/bge-small-en-v1.5"
batch_size = 32

[storage]
base = "~/.helix/data"  # Override data location if needed
```

## Path Helpers

```rust
use helix_config::{helix_home, helix_config_dir, helix_data_dir};

// Get the helix home directory
let home = helix_home();  // ~/.helix

// Get specific subdirectories
let config_dir = helix_config_dir();  // ~/.helix/config
let data_dir = helix_data_dir();      // ~/.helix/data

// Tool-specific data directories
let docs_dir = helix_data_dir().join("docs");    // ~/.helix/data/docs
let index_dir = helix_data_dir().join("index");  // ~/.helix/data/index
```

## GitHub Token Detection

The crate provides automatic GitHub token detection:

```rust
use helix_config::detect_github_token;

if let Some(token) = detect_github_token() {
    // Token found from: GITHUB_TOKEN, GH_TOKEN, config, or `gh auth token`
}
```

Detection order:
1. `GITHUB_TOKEN` environment variable
2. `GH_TOKEN` environment variable
3. `github.token` in config files
4. `gh auth token` command output

## Environment Variables

```bash
# Override helix home directory
export HELIX_HOME=~/my-helix

# Shared settings
export HELIX_GITHUB_TOKEN=ghp_xxx
export HELIX_EMBEDDING_MODEL=jina-embeddings-v3

# Tool-specific overrides (using __ for nesting)
export HELIX_DOCS__INGEST__CONCURRENCY=10
export HELIX_MAP__INDEXING__LANGUAGES=rust,typescript
```

## Consumers

| Crate | Tool Name | Config File |
|-------|-----------|-------------|
| `hbd` | `hbd` | `~/.helix/config/hbd.toml` |
| `helix-docs` | `helix-docs` | `~/.helix/config/helix-docs.toml` |
| `helix-map` | `helix-map` | `~/.helix/config/helix-map.toml` |
| `helix-repo` | `helix-repo` | `~/.helix/config/helix-repo.toml` |
| `helix-mail` | `helix-mail` | `~/.helix/config/helix-mail.toml` |

## Specifications

- [specs/requirements.md][requirements] - Requirements in EARS notation
- [specs/design.md][design] - Design decisions and API details

<!-- Links -->
[config-docs]: https://kevinmichaelchen.github.io/helix-tools/docs/configuration
[requirements]: ./specs/requirements.md
[design]: ./specs/design.md
