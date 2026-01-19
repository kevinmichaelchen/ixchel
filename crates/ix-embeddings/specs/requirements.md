# ix-embeddings: Requirements Specification

**Document:** requirements.md\
**Status:** Active (2026-01-06)\
**Author:** Kevin Chen

## Vision

ix-embeddings provides shared embedding infrastructure for Ixchel, enabling semantic search across decisions and issues using local inference.

## User Stories

### For Tool Developers

```
As an Ixchel developer,
I want a simple embedding API,
So that I can add semantic search without managing fastembed directly.
```

**Acceptance Criteria:**

- Single `Embedder` struct with `embed()` and `embed_batch()`
- Configuration via ix-config (model, batch size)
- Works offline (no external APIs)

### For End Users

```
As a developer using Ixchel,
I want semantic search to work offline,
So that I can search decisions/issues without internet access.
```

**Acceptance Criteria:**

- CPU-only inference (no GPU required)
- Model cached locally after first download
- Consistent results across machines

## Functional Requirements

### FR-1: Single Text Embedding

- **EARS:** The system SHALL embed a single text string into a vector.
- **Input:** Text string (any length)
- **Output:** `Vec<f32>` of model dimension (e.g., 384)

### FR-2: Batch Embedding

- **EARS:** The system SHALL embed multiple texts efficiently in a single call.
- **Input:** Slice of text strings
- **Output:** `Vec<Vec<f32>>` matching input order

### FR-3: Model Configuration

- **EARS:** The system SHALL support model selection via ix-config.
- **Config path:** `~/.ixchel/config/config.toml` → `[embedding].model`
- **Default:** `BAAI/bge-small-en-v1.5`

### FR-4: Normalization

- **EARS:** The system SHALL normalize embeddings to unit length by default.
- **Why:** Required for cosine similarity to work correctly

### FR-5: Dimension Query

- **EARS:** The system SHALL expose the embedding dimension.
- **Use case:** Consumers need dimension for HelixDB vector index setup

## Non-Functional Requirements

### Performance

| Operation                | Target  |
| ------------------------ | ------- |
| First embed (cold start) | < 2s    |
| Single embed (warm)      | < 100ms |
| Batch of 32              | < 1.5s  |
| Batch of 100             | < 4s    |

### Reliability

- Handle empty strings gracefully
- Handle very long texts (truncate to model max)
- Thread-safe (`Send + Sync`)

### Compatibility

- Rust stable
- Cross-platform (Linux, macOS, Windows)
- CPU-only (no CUDA dependency)

## Supported Models

| Model                                    | Dimensions | Default |
| ---------------------------------------- | ---------- | ------- |
| `BAAI/bge-small-en-v1.5`                 | 384        | Yes     |
| `BAAI/bge-base-en-v1.5`                  | 768        | No      |
| `sentence-transformers/all-MiniLM-L6-v2` | 384        | No      |

## Out of Scope

- Cross-encoder reranking (handled by higher-level retrieval components)
- Embedding caching (consumer responsibility, stored in HelixDB)
- Non-English models (English-focused for now)

## Consumers

| Tool / Crate         | Use Case                                |
| -------------------- | --------------------------------------- |
| `ix-storage-helixdb` | Embed entities for sync + vector search |
| `demo-got`           | Demo: embed bios for semantic search    |

## See Also

- [design.md](./design.md) — Architecture and API details
- [ix-config/specs/design.md](../../ix-config/specs/design.md) — Configuration
