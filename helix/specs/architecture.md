# Architecture Specification

This document describes the technical architecture of the Helix unified knowledge system.

## System Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         User Interfaces                              │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐  ┌────────────┐ │
│  │   CLI       │  │    TUI      │  │  MCP Server │  │   Library  │ │
│  │  (helix)    │  │  (helix ui) │  │  (helixd)   │  │   (crate)  │ │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘  └─────┬──────┘ │
└─────────┼────────────────┼────────────────┼───────────────┼────────┘
          │                │                │               │
          └────────────────┴────────────────┴───────────────┘
                                    │
┌───────────────────────────────────▼─────────────────────────────────┐
│                        Core Library (helix-core)                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Entity Abstraction Layer                  │   │
│  │  ┌─────────┬─────────┬────────┬────────┬────────┬────────┐ │   │
│  │  │Decision │  Issue  │  Idea  │ Report │ Source │Citation│ │   │
│  │  └────┬────┴────┬────┴───┬────┴───┬────┴───┬────┴───┬────┘ │   │
│  │       └─────────┴────────┴────────┴────────┴────────┘       │   │
│  │                           │                                  │   │
│  │              ┌────────────▼────────────┐                    │   │
│  │              │     Entity<T> Trait     │                    │   │
│  │              │  • id, title, metadata  │                    │   │
│  │              │  • relationships        │                    │   │
│  │              │  • to_markdown/from_md  │                    │   │
│  │              │  • embedding_text       │                    │   │
│  │              └────────────┬────────────┘                    │   │
│  └───────────────────────────┼──────────────────────────────────┘   │
│                              │                                       │
│  ┌───────────────────────────▼──────────────────────────────────┐   │
│  │                    Storage Layer                              │   │
│  │  ┌────────────────┐  ┌────────────────┐  ┌────────────────┐ │   │
│  │  │  FileStorage   │  │  GraphStorage  │  │ VectorStorage  │ │   │
│  │  │  (Markdown)    │  │  (HelixDB)     │  │  (HNSW)        │ │   │
│  │  └───────┬────────┘  └───────┬────────┘  └───────┬────────┘ │   │
│  │          │                   │                   │           │   │
│  │          └───────────────────┴───────────────────┘           │   │
│  │                              │                                │   │
│  │              ┌───────────────▼───────────────┐               │   │
│  │              │      Unified Storage API      │               │   │
│  │              │  • create, read, update, delete│              │   │
│  │              │  • search (graph + vector)    │               │   │
│  │              │  • traverse relationships     │               │   │
│  │              │  • sync (files ↔ db)          │               │   │
│  │              └───────────────────────────────┘               │   │
│  └───────────────────────────────────────────────────────────────┘   │
│                                                                      │
│  ┌───────────────────────────────────────────────────────────────┐   │
│  │                    Supporting Services                         │   │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐   │   │
│  │  │  Embedder   │  │  Validator  │  │  Delta Detector     │   │   │
│  │  │(helix-embed)│  │  (schemas)  │  │  (incremental sync) │   │   │
│  │  └─────────────┘  └─────────────┘  └─────────────────────┘   │   │
│  └───────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────────┘
                                    │
┌───────────────────────────────────▼─────────────────────────────────┐
│                        Persistence Layer                             │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    Filesystem (Git-tracked)                  │   │
│  │  .helix/                                                     │   │
│  │  ├── decisions/*.md    (source of truth)                    │   │
│  │  ├── issues/*.md                                            │   │
│  │  ├── ideas/*.md                                             │   │
│  │  ├── reports/*.md                                           │   │
│  │  ├── sources/*.md                                           │   │
│  │  └── citations/*.md                                         │   │
│  └─────────────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                    HelixDB (Cache/Index)                     │   │
│  │  .helix/data/helix.db/   (gitignored, rebuildable)          │   │
│  │  ├── data.mdb           (LMDB graph + vectors)              │   │
│  │  └── lock.mdb                                               │   │
│  └─────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────────┘
```

## Crate Structure

```
helix-tools/
├── helix/                    # Main CLI binary
│   ├── src/
│   │   ├── main.rs          # Entry point, command dispatch
│   │   ├── commands/        # CLI command implementations
│   │   │   ├── create.rs
│   │   │   ├── show.rs
│   │   │   ├── list.rs
│   │   │   ├── update.rs
│   │   │   ├── search.rs
│   │   │   ├── graph.rs
│   │   │   ├── context.rs
│   │   │   └── ...
│   │   └── output.rs        # Formatting (table, JSON, markdown)
│   └── Cargo.toml
│
├── helix-core/              # Core library (the brain)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── entity/          # Entity type definitions
│   │   │   ├── mod.rs       # Entity trait
│   │   │   ├── decision.rs
│   │   │   ├── issue.rs
│   │   │   ├── idea.rs
│   │   │   ├── report.rs
│   │   │   ├── source.rs
│   │   │   └── citation.rs
│   │   ├── storage/         # Storage implementations
│   │   │   ├── mod.rs       # StorageBackend trait
│   │   │   ├── file.rs      # Markdown file I/O
│   │   │   ├── graph.rs     # HelixDB graph operations
│   │   │   ├── vector.rs    # HNSW vector operations
│   │   │   └── unified.rs   # Coordinated storage
│   │   ├── search/          # Search implementations
│   │   │   ├── mod.rs
│   │   │   ├── semantic.rs  # Vector similarity
│   │   │   ├── filter.rs    # Property filters
│   │   │   └── hybrid.rs    # Combined search
│   │   ├── graph/           # Graph algorithms
│   │   │   ├── mod.rs
│   │   │   ├── traverse.rs  # BFS, DFS, path finding
│   │   │   ├── cycles.rs    # Cycle detection
│   │   │   └── layout.rs    # DOT generation
│   │   ├── sync/            # File ↔ DB synchronization
│   │   │   ├── mod.rs
│   │   │   ├── delta.rs     # Change detection
│   │   │   ├── manifest.rs  # Sync state tracking
│   │   │   └── reconcile.rs # Conflict resolution
│   │   ├── context/         # AI context generation
│   │   │   ├── mod.rs
│   │   │   └── builder.rs
│   │   └── config.rs        # Configuration management
│   └── Cargo.toml
│
├── helix-tui/               # Terminal UI
│   ├── src/
│   │   ├── main.rs
│   │   ├── app.rs           # Application state
│   │   ├── ui/              # UI components (ratatui)
│   │   │   ├── mod.rs
│   │   │   ├── browser.rs   # Entity list view
│   │   │   ├── detail.rs    # Single entity view
│   │   │   ├── graph.rs     # Graph visualization
│   │   │   ├── search.rs    # Search interface
│   │   │   └── dashboard.rs # Health/stats view
│   │   └── events.rs        # Input handling
│   └── Cargo.toml
│
├── helix-mcp/               # MCP server for AI tools
│   ├── src/
│   │   ├── main.rs
│   │   └── tools/           # MCP tool implementations
│   └── Cargo.toml
│
├── shared/
│   ├── helix-db/            # Graph-vector database (existing)
│   ├── helix-embeddings/    # Embedding providers (existing)
│   └── helix-config/        # Configuration management (existing)
│
├── hbd/                     # Legacy issue tracker (to be absorbed)
└── helix-decisions/         # Legacy decision tracker (to be absorbed)
```

## Core Abstractions

### Entity Trait

```rust
/// Common interface for all entity types
pub trait Entity: Sized + Send + Sync {
    /// Type prefix for IDs (e.g., "dec", "iss")
    const PREFIX: &'static str;

    /// Directory name under .helix/
    const DIRECTORY: &'static str;

    /// Unique identifier
    fn id(&self) -> &str;

    /// Human-readable title
    fn title(&self) -> &str;

    /// Base metadata (common to all entities)
    fn metadata(&self) -> &EntityMetadata;
    fn metadata_mut(&mut self) -> &mut EntityMetadata;

    /// Relationships to other entities
    fn relationships(&self) -> &[Relationship];
    fn add_relationship(&mut self, rel: Relationship);
    fn remove_relationship(&mut self, target_id: &str, rel_type: RelationType);

    /// Convert to/from Markdown with YAML frontmatter
    fn to_markdown(&self) -> String;
    fn from_markdown(content: &str, file_path: PathBuf) -> Result<Self>;

    /// Text used for embedding generation
    fn embedding_text(&self) -> String {
        format!("{}\n\n{}\n\nTags: {}",
            self.title(),
            self.body(),
            self.metadata().tags.join(", ")
        )
    }

    /// Markdown body content
    fn body(&self) -> &str;

    /// Validate entity-specific rules
    fn validate(&self) -> Result<()>;
}

/// Common metadata for all entities
pub struct EntityMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub created_by_type: CreatorType,
    pub agent_id: Option<String>,
    pub session_id: Option<String>,
    pub tags: Vec<String>,
    pub external_ref: Option<String>,
    pub uuid: String,
    pub content_hash: String,
}

/// Relationship between entities
pub struct Relationship {
    pub target_id: String,
    pub rel_type: RelationType,
}

/// All possible relationship types
pub enum RelationType {
    // Universal
    RelatesTo,

    // Blocking/dependency
    Blocks,
    DependsOn,

    // Evolution
    Supersedes,
    Amends,
    EvolvesInto,
    Spawns,

    // Citation
    Cites,
    Quotes,
    Supports,
    Contradicts,

    // Aggregation
    Summarizes,
    Addresses,
    Implements,

    // Provenance
    SpawnedBy,
    FromSource,
    UsedIn,
}
```

### Storage Backend Trait

```rust
/// Unified storage interface
#[async_trait]
pub trait StorageBackend: Send + Sync {
    /// Create a new entity
    async fn create<E: Entity>(&self, entity: &E) -> Result<()>;

    /// Read entity by ID
    async fn read<E: Entity>(&self, id: &str) -> Result<E>;

    /// Update existing entity
    async fn update<E: Entity>(&self, entity: &E) -> Result<()>;

    /// Delete entity by ID
    async fn delete<E: Entity>(&self, id: &str) -> Result<()>;

    /// List all entities of a type with optional filters
    async fn list<E: Entity>(&self, filters: &Filters) -> Result<Vec<E>>;

    /// Semantic search across entity types
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>>;

    /// Traverse relationships from an entity
    async fn traverse(&self, id: &str, query: &TraverseQuery) -> Result<Graph>;

    /// Resolve partial ID to full ID
    async fn resolve_id(&self, partial: &str) -> Result<String>;

    /// Synchronize files with database
    async fn sync(&self) -> Result<SyncReport>;
}

/// Search query parameters
pub struct SearchQuery {
    pub text: String,
    pub entity_types: Vec<EntityType>,  // Empty = all types
    pub filters: Filters,
    pub limit: usize,
    pub offset: usize,
}

/// Property-based filters
pub struct Filters {
    pub status: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub created_by: Option<String>,
    pub priority: Option<Vec<u8>>,
    // ... entity-specific filters
}

/// Graph traversal query
pub struct TraverseQuery {
    pub direction: Direction,           // Outgoing, Incoming, Both
    pub relationship_types: Vec<RelationType>,  // Empty = all
    pub max_depth: usize,
    pub include_entity_types: Vec<EntityType>,  // Empty = all
}
```

## Data Flow

### Write Path

```
User creates entity (CLI/TUI)
         │
         ▼
┌─────────────────────┐
│  Validate entity    │
│  (schema + rules)   │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Generate ID        │
│  (if new)           │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Write Markdown     │ ◄── Source of truth
│  file to .helix/    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Generate embedding │
│  (async background) │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Update HelixDB     │ ◄── Index/cache
│  (graph + vector)   │
└─────────────────────┘
```

### Read Path (Search)

```
User searches "database performance"
         │
         ▼
┌─────────────────────┐
│  Generate query     │
│  embedding          │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Vector search      │
│  (HNSW k-NN)        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Apply filters      │
│  (status, type,     │
│   tags, date)       │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Rank & return      │
│  results            │
└─────────────────────┘
```

### Read Path (Graph Traversal)

```
User requests "helix graph dec-42 --depth 3"
         │
         ▼
┌─────────────────────┐
│  Resolve dec-42     │
│  to node ID         │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  BFS/DFS traverse   │
│  up to depth 3      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Collect nodes      │
│  and edges          │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Format output      │
│  (DOT, JSON, tree)  │
└─────────────────────┘
```

### Sync Path

```
User runs "helix sync" or daemon detects file change
         │
         ▼
┌─────────────────────┐
│  Scan .helix/       │
│  directories        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Compare with       │
│  manifest           │
│  (mtime, size, hash)│
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│  Detect delta:      │
│  added, modified,   │
│  deleted, renamed   │
└──────────┬──────────┘
           │
           ├──────────────────┐
           │                  │
           ▼                  ▼
┌─────────────────┐  ┌─────────────────┐
│  Re-embed       │  │  Update graph   │
│  changed files  │  │  nodes/edges    │
└────────┬────────┘  └────────┬────────┘
         │                    │
         └──────────┬─────────┘
                    │
                    ▼
         ┌─────────────────────┐
         │  Update manifest    │
         └─────────────────────┘
```

## Graph Schema

### Node Labels

```
DECISION    - Architecture decisions
ISSUE       - Work items
IDEA        - Proposals
REPORT      - Analysis documents
SOURCE      - External references
CITATION    - Specific quotes
```

### Edge Types

```
RELATES_TO    - General association (bidirectional)
BLOCKS        - A blocks B
DEPENDS_ON    - A depends on B
SUPERSEDES    - A replaces B
AMENDS        - A modifies B
EVOLVES_INTO  - A became B
SPAWNS        - A created B
CITES         - A references B
QUOTES        - A excerpts B
SUPPORTS      - A provides evidence for B
CONTRADICTS   - A conflicts with B
SUMMARIZES    - A condenses B
ADDRESSES     - A responds to B
IMPLEMENTS    - A implements B
SPAWNED_BY    - Inverse of SPAWNS
FROM_SOURCE   - Citation's source
USED_IN       - Citation used in report/decision
```

### Secondary Indices

```
id          - Entity ID (unique)
uuid        - Content-based UUID
vector_id   - Link to HNSW vector
status      - For filtering
type        - Entity-specific type (bug, feature, etc.)
created_at  - For time-based queries
tags        - For tag filtering
```

### Vector Index

```
HNSW Configuration:
  m: 16                    # Max connections per node
  ef_construction: 200     # Build-time search depth
  ef_search: 64            # Query-time search depth
  dimensions: 384          # BGE-small-en-v1.5
  distance: cosine         # Normalized vectors
```

## Configuration

### .helix/config.toml

```toml
[helix]
version = "1"

[embedding]
provider = "fastembed"              # or "candle"
model = "BAAI/bge-small-en-v1.5"
batch_size = 32

[storage]
db_max_size_gb = 2

[sync]
auto_sync = true                    # Watch for file changes
sync_interval_ms = 1000

[hooks]
immutable_decisions = true          # Enforce decision immutability
pre_commit = true                   # Install git hooks

[search]
default_limit = 10
hybrid_alpha = 0.7                  # Vector vs keyword weight

[context]
max_tokens = 8000                   # Context generation limit
include_relationships = true
```

## Error Handling

### Error Types

```rust
pub enum HelixError {
    // Entity errors
    EntityNotFound { id: String, entity_type: EntityType },
    AmbiguousId { partial: String, matches: Vec<String> },
    InvalidEntity { id: String, reason: String },
    ImmutableEntity { id: String, field: String },

    // Relationship errors
    CycleDetected { path: Vec<String> },
    InvalidRelationship { from: String, to: String, reason: String },

    // Storage errors
    FileNotFound { path: PathBuf },
    ParseError { path: PathBuf, reason: String },
    DatabaseError { reason: String },
    SyncConflict { id: String, file_version: String, db_version: String },

    // Search errors
    EmbeddingError { reason: String },

    // Configuration errors
    ConfigNotFound,
    InvalidConfig { reason: String },
}
```

### Exit Codes

```
0  - Success
1  - General error
2  - Invalid input / configuration
3  - Entity not found / ambiguous ID
4  - Cycle detected
5  - Sync conflict
6  - Database error
7  - Embedding error
```

## Concurrency Model

### Single Process (CLI)

- Sequential operations
- Read-write transactions on HelixDB
- File writes are atomic (write to temp, rename)

### Daemon Mode (helixd)

```
┌─────────────────────────────────────────┐
│                 helixd                   │
│  ┌─────────────────────────────────┐    │
│  │     File Watcher (notify-rs)    │    │
│  └──────────────┬──────────────────┘    │
│                 │                        │
│                 ▼                        │
│  ┌─────────────────────────────────┐    │
│  │       Sync Queue (mpsc)         │    │
│  └──────────────┬──────────────────┘    │
│                 │                        │
│                 ▼                        │
│  ┌─────────────────────────────────┐    │
│  │    Sync Worker (single writer)  │    │
│  │    • Delta detection            │    │
│  │    • Embedding generation       │    │
│  │    • HelixDB updates            │    │
│  └─────────────────────────────────┘    │
│                                          │
│  ┌─────────────────────────────────┐    │
│  │      MCP Server (optional)       │    │
│  │      • Read-only queries         │    │
│  │      • Queued writes             │    │
│  └─────────────────────────────────┘    │
└──────────────────────────────────────────┘
```

- Single writer thread owns write transactions
- CLI/MCP reads are concurrent (LMDB MVCC)
- File watcher debounces rapid changes
- Embedding generation is async/parallel

## Security Considerations

### File Access

- All operations scoped to `.helix/` directory
- No execution of file contents
- YAML parsing uses safe subset (no arbitrary objects)

### Git Hooks

- Hooks are opt-in (require explicit installation)
- Hook scripts are validated before installation
- Can be bypassed with `--no-verify`

### External References

- URLs in sources are not auto-fetched
- `archived_at` links are informational only
- No automatic network requests

### Sensitive Data

- No secrets storage mechanism
- Warning if entity content looks like credentials
- Embedding model runs locally (no API calls)
