# helix-map Tasks

## Phase 0: Tracer Bullet (Done)

- [x] Scan repository files (Rust-only)
- [x] Extract a minimal symbol index (functions/types/impls)
- [x] Persist an index to `.helix-map/index.json`
- [x] Render a compact “skeleton” view for LLM context

## Phase 1: Usability & Correctness

- [ ] Add include/exclude patterns and language filters
- [ ] Add stable symbol IDs (hash of qualified name + span)
- [ ] Improve module tree + re-export handling (`pub use`)
- [ ] Add doc comment capture into skeleton output

## Phase 2: Multi-Language Extraction

- [ ] Add TypeScript/JavaScript extractor (Tree-sitter queries)
- [ ] Add Svelte extractor (script exports + component API)
- [ ] Add extractor registry + per-language configuration

## Phase 3: Better Graphs & Storage

- [ ] Add import/export graph edges
- [ ] Add best-effort call edges for navigation
- [ ] Replace JSON storage with a HelixDB backend (rebuildable)
- [ ] Add incremental updates (watch mode / changed files only)
