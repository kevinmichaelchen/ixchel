# demo-got

A Game of Thrones family tree demo showcasing graph and vector search capabilities with pluggable storage backends.

## Overview

This crate demonstrates how to use embedded graph databases for storing and querying family relationships. It ingests ~30 characters from Game of Thrones (Houses Stark, Targaryen, Baratheon, Tully, Lannister) and supports both graph traversal queries and semantic search over character biographies.

**Supported backends:**

- **SurrealDB** (default) - Pure Rust embedded database with SurrealKV storage
- **HelixDB** - High-performance graph database with LMDB storage

## Installation

```bash
cargo build -p demo-got
```

## Usage

### Backend Selection

Use the `--backend` flag to choose your storage backend:

```bash
# SurrealDB (default)
cargo run -p demo-got -- ingest

# HelixDB
cargo run -p demo-got -- --backend helixdb ingest
```

Each backend uses its own database directory to avoid conflicts:

- SurrealDB: `.data-surrealdb/`
- HelixDB: `.data-helixdb/`

### Ingest Data

Load the family tree from YAML into the database with embeddings:

```bash
cargo run -p demo-got -- ingest
```

This creates a database directory inside the crate with the persisted graph data. By default, it also:

- Loads character biographies from `data/*.md` files
- Generates embeddings using the local embedding model
- Stores vectors for semantic search

For faster iteration during development, skip embedding generation:

```bash
cargo run -p demo-got -- ingest --skip-embeddings
```

To clear existing data and re-ingest:

```bash
cargo run -p demo-got -- ingest --clear
```

### Query Commands

**Find ancestors** (follows PARENT_OF edges in reverse):

```bash
cargo run -p demo-got -- query ancestors jon-snow
```

Output reveals Jon's true parentage (R+L=J):

```
Ancestors of jon-snow:
  Lyanna Stark "The She-Wolf" (House Stark)
  Rhaegar Targaryen "The Last Dragon" (House Targaryen)
    Rickard Stark (House Stark)
    Aerys II Targaryen "The Mad King" (House Targaryen)
    ...
```

**Find descendants** (follows PARENT_OF edges forward):

```bash
cargo run -p demo-got -- query descendants ned-stark
```

**List house members:**

```bash
cargo run -p demo-got -- query house stark
```

**Get person details with immediate family:**

```bash
cargo run -p demo-got -- query person ned-stark
```

**Show database statistics:**

```bash
cargo run -p demo-got -- stats
```

### Semantic Search

Search for characters using natural language queries:

```bash
cargo run -p demo-got -- search "the mad king"
```

Output:

```
Search results for: "the mad king"

1. Aerys II Targaryen "The Mad King" (House Targaryen) - score: 0.847
2. Viserys Targaryen (House Targaryen) - score: 0.623
3. Daenerys Targaryen "Mother of Dragons" (House Targaryen) - score: 0.598
```

More examples:

```bash
# Find dragon-related characters
cargo run -p demo-got -- search "mother of dragons"

# Find characters by their story arcs
cargo run -p demo-got -- search "characters who died tragically"

# Limit results
cargo run -p demo-got -- search "dragon riders" --limit 3
```

### JSON Output

Add `--json` for machine-readable output:

```bash
cargo run -p demo-got -- --json query ancestors jon-snow
```

## Data Model

### Nodes (Person)

| Property  | Type     | Description                          |
| --------- | -------- | ------------------------------------ |
| id        | String   | Unique identifier (e.g., "jon-snow") |
| name      | String   | Full name (e.g., "Jon Snow")         |
| house     | String   | House affiliation                    |
| titles    | String[] | Array of titles held                 |
| alias     | String?  | Common nickname                      |
| is_alive  | bool     | Living or deceased                   |
| embedding | float[]  | Vector embedding (if generated)      |

### Edges (Relationships)

| Label      | Direction       | Description                |
| ---------- | --------------- | -------------------------- |
| PARENT_OF  | Parent -> Child | Biological/adoptive parent |
| SPOUSE_OF  | Bidirectional   | Marriage relationship      |
| SIBLING_OF | Bidirectional   | Sibling relationship       |

## Architecture

```
apps/demo-got/
├── src/
│   ├── main.rs           # CLI entry point (clap)
│   ├── lib.rs            # Module exports
│   ├── backend.rs        # GotBackend trait definition
│   ├── types.rs          # Person, House, RelationType, SearchResult
│   ├── error.rs          # GotError enum
│   ├── loader.rs         # YAML + bio markdown parsing
│   ├── query.rs          # BFS graph traversal (generic over backend)
│   └── storage/
│       ├── mod.rs        # Backend exports
│       ├── helixdb.rs    # HelixDB implementation
│       └── surrealdb.rs  # SurrealDB implementation
└── data/
    ├── westeros.yaml     # Family tree seed data
    └── *.md              # Character biographies (30 files)
```

### Backend Abstraction

The `GotBackend` trait defines the storage interface:

```rust
pub trait GotBackend: Send + Sync {
    fn new(db_path: &Path) -> Result<Self>;
    fn exists(db_path: &Path) -> bool;
    fn clear(&self) -> Result<()>;
    fn ingest(&mut self, tree: &FamilyTree) -> Result<IngestStats>;
    fn search_semantic(&self, embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>>;
    fn get_person(&self, node_id: &str) -> Result<Person>;
    fn get_incoming_neighbors(&self, node_id: &str, rel: RelationType) -> Result<Vec<String>>;
    fn get_outgoing_neighbors(&self, node_id: &str, rel: RelationType) -> Result<Vec<String>>;
    // ... more methods
}
```

### Vector Search

Semantic search uses:

- **HNSW index**: `m=16`, `ef_construction=128-150`, `ef_search=64`
- **Embedding model**: `BAAI/bge-small-en-v1.5` (384 dimensions)
- **Composite text**: `{name} ({alias})\nTitles: {titles}\n\n{bio}`

## License

MIT

## Kiro Specs

- [Requirements][requirements]
- [Design][design]
- [Tasks][tasks]

<!-- Links -->

[requirements]: specs/requirements.md
[design]: specs/design.md
[tasks]: specs/tasks.md
