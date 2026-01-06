# Design

This document describes the design decisions and implementation details for `helix-repo`.

## Overview

`helix-repo` manages git repository clones for the helix-tools ecosystem. It provides both a CLI and a library API for cloning repositories to a standardized directory structure.

## Design Goals

1. **Single Responsibility** - Clone and manage git repos, nothing more
2. **Composable** - Library API for use by other helix-tools
3. **Simple** - Git-only initially (like git-grab), VCS abstraction for future
4. **Predictable** - Deterministic paths from URLs
5. **Minimal Dependencies** - Delegate to `git` CLI, don't embed libgit2

## Inspiration

| Tool | What We Take | What We Skip |
|------|--------------|--------------|
| [git-grab](https://github.com/wezm/git-grab) | URL parsing, simplicity, Rust patterns | Clipboard features |
| [ghq](https://github.com/x-motemen/ghq) | VCS abstraction concept, multiple roots | git-config integration, multi-VCS |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CLI Layer                                       │
│                                                                              │
│   helix-repo clone, list, info, remove, root                                │
│   - Rust CLI with clap                                                       │
│   - All commands support --json for AI agents                                │
│   - Commands delegate to RepositoryManager                                   │
└────────────────────────────────────────┬────────────────────────────────────┘
                                         │
┌────────────────────────────────────────┴────────────────────────────────────┐
│                         Library API (lib.rs)                                 │
│                                                                              │
│   RepositoryManager                                                          │
│   - clone(url) -> Result<PathBuf>                                           │
│   - find(name) -> Result<Option<PathBuf>>                                   │
│   - list() -> Result<Vec<Repository>>                                       │
│   - remove(name) -> Result<()>                                              │
│                                                                              │
│   Orchestrates operations, uses traits for I/O                              │
└────────────────────────────────────────┬────────────────────────────────────┘
                                         │
┌────────────────────────────────────────┴────────────────────────────────────┐
│                            Domain Layer                                      │
│                                                                              │
│   Repository        RepoUrl           RepoPath                              │
│   - name            - original        - root                                │
│   - url             - normalized      - domain                              │
│   - path            - domain          - owner                               │
│   - cloned_at       - owner           - repo                                │
│                     - repo            - full_path()                         │
│                                                                              │
│   Pure types with no I/O dependencies                                        │
└────────────────────────────────────────┬────────────────────────────────────┘
                                         │
┌────────────────────────────────────────┴────────────────────────────────────┐
│                         Port Traits (Interfaces)                             │
│                                                                              │
│   trait GitClient                      trait RepositoryStore                 │
│   - clone(url, path) -> Result<()>     - list() -> Result<Vec<Repository>>  │
│   - is_repo(path) -> bool              - save(repo) -> Result<()>           │
│                                        - remove(name) -> Result<()>         │
│                                                                              │
│   Abstract interfaces for testability                                        │
└────────────────────────────────────────┬────────────────────────────────────┘
                                         │
          ┌──────────────────────────────┴──────────────────────────────┐
          v                                                              v
┌─────────────────────────┐                              ┌─────────────────────────┐
│   GitCliClient          │                              │   FileSystemStore       │
│                         │                              │                         │
│   - Shells out to git   │                              │   - Scans directories   │
│   - Parses exit codes   │                              │   - No metadata file    │
│   - Streams output      │                              │   - Pure filesystem     │
└─────────────────────────┘                              └─────────────────────────┘
```

---

## Module Structure

```
helix-repo/
├── Cargo.toml
├── README.md
├── specs/
│   ├── requirements.md
│   ├── design.md
│   └── tasks.md
└── src/
    ├── main.rs              # CLI entry point
    ├── lib.rs               # Public API exports
    │
    ├── cli/                 # CLI layer
    │   ├── mod.rs           # Cli struct, Commands enum
    │   ├── clone.rs         # Clone command
    │   ├── list.rs          # List command
    │   ├── info.rs          # Info command
    │   ├── remove.rs        # Remove command
    │   └── root.rs          # Root command
    │
    ├── domain/              # Pure domain types
    │   ├── mod.rs
    │   ├── repository.rs    # Repository struct
    │   └── url.rs           # RepoUrl parsing
    │
    ├── ports/               # Trait definitions
    │   ├── mod.rs
    │   ├── git.rs           # GitClient trait
    │   └── store.rs         # RepositoryStore trait
    │
    ├── adapters/            # Trait implementations
    │   ├── mod.rs
    │   ├── git_cli.rs       # Shell out to git
    │   └── fs_store.rs      # Filesystem scanning
    │
    ├── manager.rs           # RepositoryManager
    ├── config.rs            # Configuration loading
    ├── path.rs              # Path calculation
    └── error.rs             # Error types
```

---

## CLI Structure

```rust
#[derive(Parser)]
#[command(name = "helix-repo")]
pub struct Cli {
    /// Root directory for repositories (default: ~/dev)
    #[arg(long, global = true, env = "HELIX_REPO_ROOT")]
    pub root: Option<PathBuf>,
    
    /// Output as JSON
    #[arg(long, global = true)]
    pub json: bool,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Clone(CloneArgs),
    List(ListArgs),
    Info(InfoArgs),
    Remove(RemoveArgs),
    Root,
}

#[derive(Args)]
pub struct CloneArgs {
    /// Repository URL (HTTPS, SSH, or schemeless)
    pub url: String,
    
    /// Shallow clone (depth 1)
    #[arg(long)]
    pub shallow: bool,
    
    /// Clone specific branch
    #[arg(long)]
    pub branch: Option<String>,
    
    /// Preview without cloning
    #[arg(long)]
    pub dry_run: bool,
}
```

---

## Data Model

### Repository

```rust
/// A cloned repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    /// Display name (e.g., "facebook/react")
    pub name: String,
    /// Original clone URL
    pub url: String,
    /// Local filesystem path
    pub path: PathBuf,
    /// Domain (e.g., "github.com")
    pub domain: String,
    /// Owner/org (e.g., "facebook")
    pub owner: String,
    /// Repository name (e.g., "react")
    pub repo: String,
}
```

### RepoUrl

```rust
/// Parsed and normalized repository URL
#[derive(Debug, Clone)]
pub struct RepoUrl {
    /// Original input string
    pub original: String,
    /// Normalized URL (always https://)
    pub normalized: Url,
    /// Domain extracted from URL
    pub domain: String,
    /// Owner/org from path
    pub owner: String,
    /// Repository name from path
    pub repo: String,
}

impl RepoUrl {
    /// Parse a URL string, handling various formats
    pub fn parse(input: &str) -> Result<Self, UrlError>;
    
    /// Get the clone path relative to root
    pub fn relative_path(&self) -> PathBuf;
}
```

---

## URL Parsing

Inspired by git-grab's approach, handle multiple URL formats:

### Supported Formats

| Format | Example | Normalized |
|--------|---------|------------|
| HTTPS | `https://github.com/facebook/react` | `https://github.com/facebook/react` |
| HTTPS with .git | `https://github.com/facebook/react.git` | `https://github.com/facebook/react` |
| SSH | `git@github.com:facebook/react.git` | `https://github.com/facebook/react` |
| SSH with port | `git@github.com:22:facebook/react.git` | `https://github.com/facebook/react` |
| Schemeless | `github.com/facebook/react` | `https://github.com/facebook/react` |
| Git protocol | `git://github.com/facebook/react` | `https://github.com/facebook/react` |

### Parsing Algorithm

```rust
pub fn parse(input: &str) -> Result<RepoUrl, UrlError> {
    // 1. Try parsing as standard URL
    if let Ok(url) = Url::parse(input) {
        return Self::from_url(url);
    }
    
    // 2. Check if it looks like SSH (contains @ before :)
    if looks_like_ssh(input) {
        let normalized = normalize_ssh(input)?;
        return Self::from_url(normalized);
    }
    
    // 3. If contains a dot, assume schemeless HTTPS
    if input.contains('.') {
        let with_scheme = format!("https://{}", input);
        return Self::parse(&with_scheme);
    }
    
    Err(UrlError::InvalidFormat(input.to_string()))
}
```

### Path Extraction

```rust
impl RepoUrl {
    /// Extract domain/owner/repo path
    pub fn relative_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(&self.domain);
        path.push(&self.owner);
        path.push(&self.repo);
        path
    }
}
```

---

## Port Traits

### GitClient

```rust
/// Abstraction over git operations
#[async_trait]
pub trait GitClient: Send + Sync {
    /// Clone a repository to a path
    async fn clone(&self, url: &str, dest: &Path, options: &CloneOptions) -> Result<()>;
    
    /// Check if a path is a git repository
    fn is_repository(&self, path: &Path) -> bool;
    
    /// Get the remote URL of a repository
    fn remote_url(&self, path: &Path) -> Result<Option<String>>;
}

#[derive(Debug, Clone, Default)]
pub struct CloneOptions {
    pub shallow: bool,
    pub branch: Option<String>,
    pub recursive: bool,
}
```

### RepositoryStore

```rust
/// Abstraction over repository storage/discovery
pub trait RepositoryStore: Send + Sync {
    /// List all repositories under root
    fn list(&self) -> Result<Vec<Repository>>;
    
    /// Find a repository by name pattern
    fn find(&self, pattern: &str) -> Result<Option<Repository>>;
    
    /// Check if a repository exists
    fn exists(&self, name: &str) -> bool;
    
    /// Remove a repository
    fn remove(&self, name: &str) -> Result<()>;
}
```

---

## Adapters

### GitCliClient

Shells out to the `git` command:

```rust
pub struct GitCliClient;

#[async_trait]
impl GitClient for GitCliClient {
    async fn clone(&self, url: &str, dest: &Path, options: &CloneOptions) -> Result<()> {
        let mut cmd = Command::new("git");
        cmd.arg("clone");
        
        if options.shallow {
            cmd.args(["--depth", "1"]);
        }
        
        if let Some(branch) = &options.branch {
            cmd.args(["--branch", branch]);
        }
        
        if options.recursive {
            cmd.arg("--recurse-submodules");
        }
        
        cmd.arg(url);
        cmd.arg(dest);
        
        let status = cmd.status().await?;
        if !status.success() {
            return Err(Error::GitFailed(status.code()));
        }
        
        Ok(())
    }
    
    fn is_repository(&self, path: &Path) -> bool {
        path.join(".git").exists()
    }
}
```

### FileSystemStore

Discovers repositories by scanning the filesystem:

```rust
pub struct FileSystemStore {
    root: PathBuf,
}

impl RepositoryStore for FileSystemStore {
    fn list(&self) -> Result<Vec<Repository>> {
        let mut repos = Vec::new();
        
        // Walk: root/domain/owner/repo
        for domain in read_dir(&self.root)? {
            for owner in read_dir(&domain.path())? {
                for repo in read_dir(&owner.path())? {
                    if repo.path().join(".git").exists() {
                        repos.push(Repository::from_path(&self.root, &repo.path())?);
                    }
                }
            }
        }
        
        Ok(repos)
    }
    
    fn find(&self, pattern: &str) -> Result<Option<Repository>> {
        // Support: "facebook/react", "react", "github.com/facebook/react"
        for repo in self.list()? {
            if repo.matches(pattern) {
                return Ok(Some(repo));
            }
        }
        Ok(None)
    }
}
```

---

## RepositoryManager

The main entry point for library usage:

```rust
pub struct RepositoryManager<G: GitClient, S: RepositoryStore> {
    git: G,
    store: S,
    root: PathBuf,
}

impl<G: GitClient, S: RepositoryStore> RepositoryManager<G, S> {
    /// Clone a repository, returning its path
    pub async fn clone(&self, url: &str) -> Result<PathBuf> {
        let repo_url = RepoUrl::parse(url)?;
        let dest = self.root.join(repo_url.relative_path());
        
        // Already cloned?
        if dest.exists() && self.git.is_repository(&dest) {
            return Ok(dest);
        }
        
        // Create parent directories
        fs::create_dir_all(dest.parent().unwrap())?;
        
        // Clone
        self.git.clone(&repo_url.normalized.to_string(), &dest, &CloneOptions::default()).await?;
        
        Ok(dest)
    }
    
    /// Find a repository by name
    pub fn find(&self, name: &str) -> Result<Option<PathBuf>> {
        self.store.find(name).map(|opt| opt.map(|r| r.path))
    }
    
    /// List all repositories
    pub fn list(&self) -> Result<Vec<Repository>> {
        self.store.list()
    }
    
    /// Remove a repository
    pub fn remove(&self, name: &str) -> Result<()> {
        self.store.remove(name)
    }
}

impl RepositoryManager<GitCliClient, FileSystemStore> {
    /// Create with default implementations
    pub fn new(root: PathBuf) -> Self {
        Self {
            git: GitCliClient,
            store: FileSystemStore::new(root.clone()),
            root,
        }
    }
    
    /// Create from configuration
    pub fn from_config() -> Result<Self> {
        let config = Config::load()?;
        Ok(Self::new(config.root))
    }
}
```

---

## Configuration

### Config Structure

```rust
use helix_config::load_config;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub storage: StorageConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    #[serde(default = "default_root")]
    pub root: PathBuf,
}

fn default_root() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("dev")  // ~/dev
}
```

### Loading Priority

1. `--root` CLI flag (highest)
2. `HELIX_REPO_ROOT` environment variable
3. `~/.helix/config/helix-repo.toml` config file
4. Default: `~/dev` (lowest)

---

## Error Handling

```rust
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    
    #[error("Repository not found: {0}")]
    NotFound(String),
    
    #[error("Repository already exists: {0}")]
    AlreadyExists(PathBuf),
    
    #[error("Git command failed with exit code {0:?}")]
    GitFailed(Option<i32>),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidUrl(_) => 1,
            Self::NotFound(_) => 2,
            Self::AlreadyExists(_) => 3,
            Self::GitFailed(_) => 4,
            Self::Io(_) => 10,
            Self::Config(_) => 11,
        }
    }
}
```

---

## Testing Strategy

### Unit Tests

- **URL parsing** - Comprehensive tests for all formats (borrow from git-grab)
- **Path calculation** - Deterministic path from URL
- **Pattern matching** - Repository name matching

### Integration Tests

- **Mock GitClient** - Test manager without actual cloning
- **Temp directories** - Test filesystem store with real files

### End-to-End Tests (slow, optional)

- **Real cloning** - Clone actual small repos
- **Git operations** - Verify git state

---

## Future Considerations

### VCS Abstraction (v2)

```rust
pub trait VcsClient: Send + Sync {
    fn name(&self) -> &str;
    fn clone(&self, url: &str, dest: &Path, options: &CloneOptions) -> Result<()>;
    fn is_repository(&self, path: &Path) -> bool;
}

// Implementations
pub struct GitClient;
pub struct MercurialClient;
pub struct SubversionClient;
```

### Multiple Roots (v2)

```toml
[storage]
roots = [
    "~/.cache/helix/repos",
    "~/work/repos",
]
primary = "~/.cache/helix/repos"
```

### Shallow Clone Database (v2)

Track which repos were shallow cloned to enable later "unshallowing":

```rust
pub struct RepoMetadata {
    pub shallow: bool,
    pub branch: Option<String>,
    pub cloned_at: DateTime<Utc>,
    pub last_updated: Option<DateTime<Utc>>,
}
```

---

## Dependencies

| Crate | Purpose | Required |
|-------|---------|----------|
| `clap` | CLI parsing | Yes |
| `url` | URL parsing | Yes |
| `serde` | Config/JSON serialization | Yes |
| `toml` | Config file parsing | Yes |
| `thiserror` | Error types | Yes |
| `tokio` | Async runtime | Yes |
| `dirs` | Platform directories | Yes |

---

## Consumers

| Tool | Usage |
|------|-------|
| `helix-docs` | Clone repos to extract documentation |
| `helix-map` | Clone repos to index codebase structure |
| AI agents | Clone repos to search implementations |
