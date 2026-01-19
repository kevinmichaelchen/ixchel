# helix-embeddings Tasks

## Phase 0: Baseline Embedder (Done)

- [x] Provide a single `Embedder` API (`embed`, `embed_batch`, `dimension`)
- [x] Support offline CPU embeddings via fastembed
- [x] Load defaults from shared config

## Phase 1: Provider Expansion

- [ ] Add candle provider behind feature flags (metal/cuda)
- [ ] Add provider metadata reporting (model name, dimension, device)

## Phase 2: Performance & Reliability

- [ ] Add batching heuristics and backpressure for large inputs
- [ ] Add deterministic test vectors for smoke tests (no network)
