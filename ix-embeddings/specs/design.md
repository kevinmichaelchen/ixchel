# ix-embeddings Design

**Status:** Active (2026-01-19)\
**Author:** Kevin Chen

## Overview

`ix-embeddings` provides pluggable embedding infrastructure for Ixchel.

It standardizes:

- How tools turn text into vectors (`Embedder`)
- How providers are selected (`EmbeddingProvider` + `[embedding]` config)
- How embeddings are normalized (L2 unit vectors)

## Consumers

Current consumers in this workspace:

- `ix-storage-helixdb` — embed entities + query vectors
- `demo-got` — demo graph + vector search over character bios

## Architecture

```
┌────────────────────────────────────────────────────────────┐
│                   Workspace consumers                        │
│        ix-storage-helixdb        demo-got                    │
└────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌────────────────────────────────────────────────────────────┐
│                      ix-embeddings                           │
│  Embedder (public API)                                       │
│    └── Box<dyn EmbeddingProvider>                            │
└────────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┴─────────────────┐
        ▼                                   ▼
┌───────────────────────┐         ┌──────────────────────────┐
│ fastembed provider     │         │ candle provider          │
│ (default feature)      │         │ (optional feature)       │
└───────────────────────┘         └──────────────────────────┘
```

## Public API

### `EmbeddingProvider`

Providers implement the `EmbeddingProvider` trait:

- `embed(text) -> Vec<f32>`
- `embed_batch(texts) -> Vec<Vec<f32>>`
- `dimension() -> usize`
- `model_name() -> &str`
- `provider_name() -> &'static str`

### `Embedder`

`Embedder` is a thin wrapper around a boxed provider:

- `Embedder::new()` loads `[embedding]` config via `ix-config`
- `Embedder::with_config(&EmbeddingConfig)` selects a provider explicitly
- `Embedder::from_provider(Box<dyn EmbeddingProvider>)` supports injection (tests / custom)

Example:

```rust
use ix_embeddings::Embedder;

let embedder = Embedder::new()?;
let v = embedder.embed("authentication bug")?;
```

## Configuration

`ix-embeddings` reads from the shared config files:

- `~/.ixchel/config/config.toml`
- `.ixchel/config.toml` (repo root)

Schema:

```toml
[embedding]
provider = "fastembed"          # "fastembed" (default) | "candle"
model = "BAAI/bge-small-en-v1.5"
batch_size = 32
dimension = 384                # optional; validated against provider metadata when possible
```

Note: config merging uses **global precedence** (global overrides project) via `ix-config`.

## Providers

### fastembed (default)

- Feature: `fastembed` (enabled by default)
- Implementation: `fastembed::TextEmbedding`
- Model resolution:
  - Accepts exact fastembed model enum strings
  - Accepts HuggingFace-style identifiers when they match a supported model
- Output normalization: always L2-normalized in `ix-embeddings`

### candle (optional)

- Feature: `candle`
- Optional acceleration: `metal` (Apple Silicon), `cuda` (NVIDIA)
- Downloads model assets via `hf-hub` when enabled
- Uses mean pooling + L2 normalization for sentence embeddings

## Error Model

Errors are returned as `EmbeddingError` and include:

- Provider not compiled (feature missing)
- Unknown provider or model
- Init failures (download / model load / runtime)
- Embed failures
- Dimension mismatches (configured vs detected)

## Implementation Notes

- Batch behavior is provider-specific; providers advertise their preferred batch size.
- Deterministic tests should inject a custom provider via `Embedder::from_provider(...)`.
