# ix-embeddings Tasks

## Phase 0: Baseline Embedder (Done)

- [x] Provide a single `Embedder` API (`embed`, `embed_batch`, `dimension`)
- [x] Support offline CPU embeddings via fastembed
- [x] Load defaults from shared config

## Phase 1: Provider Expansion

- [x] Add candle provider behind feature flags (metal/cuda)
- [x] Add provider metadata reporting (model name, dimension, batch size)

## Phase 2: Performance & Reliability

- [ ] Add batching heuristics and backpressure for large inputs
- [ ] Add deterministic smoke tests (no network)
