# adr-search: Design Specification

**Document:** design.md  
**Status:** Concept (2026-01-05)  
**Author:** Kevin Chen

## Architecture Overview

```
┌─────────────────────────────────┐
│         adr-search CLI          │
│         (clap + main.rs)        │
└──────────────┬──────────────────┘
               │
┌──────────────▼──────────────────┐
│          ADRSearcher            │
│  • load()   - Load ADRs         │
│  • sync()   - Delta indexing    │
│  • search() - Semantic search   │
└──────────────┬──────────────────┘
               │
       ┌───────┼───────┐
       │       │       │
┌──────▼───┐ ┌─▼────┐ ┌▼─────────┐
│ Loader   │ │Embed │ │ Storage  │
│ (YAML)   │ │(fast)│ │(HelixDB) │
└──────────┘ └──────┘ └──────────┘
```

## Module Design

### types.rs
```rust
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// ADR status values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Proposed,
    Accepted,
    Superseded,
    Deprecated,
}

/// ADR metadata from YAML frontmatter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ADRMetadata {
    pub id: u32,
    pub title: String,
    pub status: Status,
    pub date: NaiveDate,
    #[serde(default)]
    pub deciders: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub supersedes: Option<u32>,
    #[serde(default)]
    pub superseded_by: Option<u32>,
}

/// Full ADR with body and computed fields
#[derive(Debug, Clone)]
pub struct ADR {
    pub metadata: ADRMetadata,
    pub body: String,
    pub file_path: PathBuf,
    pub content_hash: String,
    pub embedding: Option<Vec<f32>>,
}

/// Search result with relevance score
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub id: u32,
    pub title: String,
    pub status: Status,
    pub score: f32,
    pub tags: Vec<String>,
    pub date: NaiveDate,
    pub deciders: Vec<String>,
    pub file_path: PathBuf,
}

/// Search response envelope
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub query: String,
    pub count: usize,
    pub results: Vec<SearchResult>,
}
```

### loader.rs
```rust
use crate::types::{ADR, ADRMetadata};
use anyhow::Result;
use gray_matter::{engine::YAML, Matter};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Load all ADRs from a directory
pub fn load_adrs(dir: &Path) -> Result<Vec<ADR>> {
    let mut adrs = Vec::new();
    let matter = Matter::<YAML>::new();
    
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().map_or(false, |e| e == "md") {
            match load_adr(&path, &matter) {
                Ok(adr) => adrs.push(adr),
                Err(e) => eprintln!("Warning: Skipping {}: {}", path.display(), e),
            }
        }
    }
    
    Ok(adrs)
}

fn load_adr(path: &Path, matter: &Matter<YAML>) -> Result<ADR> {
    let content = std::fs::read_to_string(path)?;
    let parsed = matter.parse(&content);
    
    let metadata: ADRMetadata = parsed
        .data
        .ok_or_else(|| anyhow::anyhow!("Missing frontmatter"))?
        .deserialize()?;
    
    let body = parsed.content;
    let content_hash = hash_content(&content);
    
    Ok(ADR {
        metadata,
        body,
        file_path: path.to_path_buf(),
        content_hash,
        embedding: None,
    })
}

fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}
```

### embeddings.rs
```rust
use anyhow::Result;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub struct Embedder {
    model: TextEmbedding,
}

impl Embedder {
    pub fn new() -> Result<Self> {
        let model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;
        Ok(Self { model })
    }
    
    pub fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let embeddings = self.model.embed(vec![text], None)?;
        Ok(embeddings.into_iter().next().unwrap())
    }
    
    pub fn embed_batch(&self, texts: Vec<&str>) -> Result<Vec<Vec<f32>>> {
        self.model.embed(texts, None).map_err(Into::into)
    }
}
```

### storage.rs
```rust
use crate::types::ADR;
use anyhow::Result;

/// Storage trait for ADR indexing
pub trait ADRStorage {
    /// Store ADRs with embeddings
    fn index(&mut self, adrs: Vec<ADR>) -> Result<()>;
    
    /// Remove ADRs by file path
    fn remove(&mut self, paths: Vec<String>) -> Result<()>;
    
    /// Search by embedding similarity
    fn search(&self, embedding: Vec<f32>, limit: usize) -> Result<Vec<(ADR, f32)>>;
    
    /// Get stored content hashes for delta detection
    fn get_hashes(&self) -> Result<std::collections::HashMap<String, String>>;
}

/// Embedded HelixDB storage
pub struct HelixDBStorage {
    // TODO: Add HelixDB client when available
    // db: HelixDB,
}

impl HelixDBStorage {
    pub fn open() -> Result<Self> {
        // TODO: Initialize embedded HelixDB
        // let db = HelixDB::open("~/.helix/data/adr/")?;
        Ok(Self {})
    }
}

impl ADRStorage for HelixDBStorage {
    fn index(&mut self, _adrs: Vec<ADR>) -> Result<()> {
        todo!("Implement HelixDB indexing")
    }
    
    fn remove(&mut self, _paths: Vec<String>) -> Result<()> {
        todo!("Implement HelixDB removal")
    }
    
    fn search(&self, _embedding: Vec<f32>, _limit: usize) -> Result<Vec<(ADR, f32)>> {
        todo!("Implement HelixDB vector search")
    }
    
    fn get_hashes(&self) -> Result<std::collections::HashMap<String, String>> {
        todo!("Implement HelixDB hash retrieval")
    }
}
```

### delta.rs
```rust
use crate::types::ADR;
use std::collections::HashMap;

/// Delta detection result
pub struct DeltaResult {
    pub to_add: Vec<ADR>,
    pub to_remove: Vec<String>,
}

/// Compute delta between filesystem and indexed ADRs
pub fn compute_delta(
    current_adrs: Vec<ADR>,
    stored_hashes: HashMap<String, String>,
) -> DeltaResult {
    let mut to_add = Vec::new();
    let mut to_remove = Vec::new();
    
    // Track which stored paths we've seen
    let mut seen_paths: std::collections::HashSet<String> = std::collections::HashSet::new();
    
    for adr in current_adrs {
        let path = adr.file_path.to_string_lossy().to_string();
        seen_paths.insert(path.clone());
        
        match stored_hashes.get(&path) {
            Some(stored_hash) if stored_hash == &adr.content_hash => {
                // No change, skip
            }
            _ => {
                // New or changed, need to re-index
                to_add.push(adr);
            }
        }
    }
    
    // Find deleted ADRs
    for path in stored_hashes.keys() {
        if !seen_paths.contains(path) {
            to_remove.push(path.clone());
        }
    }
    
    DeltaResult { to_add, to_remove }
}
```

### searcher.rs
```rust
use crate::delta::compute_delta;
use crate::embeddings::Embedder;
use crate::loader::load_adrs;
use crate::storage::{ADRStorage, HelixDBStorage};
use crate::types::{SearchResponse, SearchResult, Status};
use anyhow::Result;
use std::path::Path;

pub struct ADRSearcher {
    storage: Box<dyn ADRStorage>,
    embedder: Embedder,
}

impl ADRSearcher {
    pub fn new() -> Result<Self> {
        let storage = Box::new(HelixDBStorage::open()?);
        let embedder = Embedder::new()?;
        Ok(Self { storage, embedder })
    }
    
    /// Load and sync ADRs from directory
    pub fn sync(&mut self, dir: &Path) -> Result<()> {
        // Load current ADRs
        let adrs = load_adrs(dir)?;
        
        // Get stored hashes
        let stored_hashes = self.storage.get_hashes()?;
        
        // Compute delta
        let delta = compute_delta(adrs, stored_hashes);
        
        // Remove deleted ADRs
        if !delta.to_remove.is_empty() {
            self.storage.remove(delta.to_remove)?;
        }
        
        // Embed and index new/changed ADRs
        if !delta.to_add.is_empty() {
            let mut adrs_with_embeddings = Vec::new();
            for mut adr in delta.to_add {
                let embedding = self.embedder.embed(&adr.body)?;
                adr.embedding = Some(embedding);
                adrs_with_embeddings.push(adr);
            }
            self.storage.index(adrs_with_embeddings)?;
        }
        
        Ok(())
    }
    
    /// Search for ADRs matching query
    pub fn search(
        &self,
        query: &str,
        limit: usize,
        status_filter: Option<Status>,
        tags_filter: Option<Vec<String>>,
    ) -> Result<SearchResponse> {
        // Embed query
        let query_embedding = self.embedder.embed(query)?;
        
        // Search storage
        let results = self.storage.search(query_embedding, limit * 2)?;  // Over-fetch for filtering
        
        // Filter and convert
        let mut search_results: Vec<SearchResult> = results
            .into_iter()
            .filter(|(adr, _)| {
                // Status filter
                if let Some(ref status) = status_filter {
                    if &adr.metadata.status != status {
                        return false;
                    }
                }
                // Tags filter
                if let Some(ref tags) = tags_filter {
                    if !tags.iter().all(|t| adr.metadata.tags.contains(t)) {
                        return false;
                    }
                }
                true
            })
            .take(limit)
            .map(|(adr, score)| SearchResult {
                id: adr.metadata.id,
                title: adr.metadata.title,
                status: adr.metadata.status,
                score,
                tags: adr.metadata.tags,
                date: adr.metadata.date,
                deciders: adr.metadata.deciders,
                file_path: adr.file_path,
            })
            .collect();
        
        search_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        Ok(SearchResponse {
            query: query.to_string(),
            count: search_results.len(),
            results: search_results,
        })
    }
}
```

## Data Flow

### First Invocation
```
1. CLI parses args (query, options)
2. ADRSearcher::new() opens HelixDB (creates if needed)
3. ADRSearcher::sync(dir) loads all ADRs from .decisions/
4. Delta check finds all ADRs are new
5. Embed all ADRs with fastembed (~2-5s)
6. Store embeddings + metadata in HelixDB
7. ADRSearcher::search(query) embeds query
8. HelixDB vector search returns ranked results
9. CLI formats and outputs results
```

### Subsequent Invocations
```
1. CLI parses args
2. ADRSearcher::new() opens existing HelixDB
3. ADRSearcher::sync(dir) loads current ADRs
4. Delta check compares hashes to stored
5. Only re-embed changed ADRs (usually 0)
6. Remove deleted ADRs from index
7. Search proceeds as normal (~100ms total)
```

## Storage Schema

### HelixDB Document Structure
```json
{
  "id": "adr/003",
  "content": "# Context and Problem Statement\n...",
  "metadata": {
    "id": 3,
    "title": "Database Migration Strategy",
    "status": "accepted",
    "date": "2026-01-04",
    "deciders": ["Alice", "Bob"],
    "tags": ["database", "migration"],
    "file_path": ".decisions/003-database-migration-strategy.md",
    "content_hash": "a1b2c3d4..."
  },
  "embedding": [0.123, 0.456, ...]
}
```

### Index Location
```
~/.helix/data/adr/
├── index/          # HelixDB vector index
└── metadata.json   # Hash cache for delta detection
```

## Embedding Model

Using `fastembed` with `AllMiniLML6V2`:
- 384 dimensions
- ~50ms per embedding (CPU)
- Good semantic understanding
- Small model size (~90MB)

## Performance Targets

| Operation | Target | Notes |
|-----------|--------|-------|
| First sync | 2-5s | Embedding 100 ADRs |
| Delta sync | < 50ms | Hash comparison |
| Query embed | 50-100ms | Single text |
| Vector search | < 50ms | HelixDB |
| Total search | < 100ms | After first run |

## Error Handling

| Error | Behavior |
|-------|----------|
| Missing directory | Exit 2 with message |
| Malformed YAML | Warn, skip file |
| HelixDB error | Exit 2 with message |
| Embedding error | Exit 2 with message |
| No results | Exit 1, show "No results" |
