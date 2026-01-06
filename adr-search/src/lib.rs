//! # adr-search
//!
//! Semantic search over Architecture Decision Records, backed by embedded HelixDB.
//!
//! ## Overview
//!
//! This crate provides a CLI and library for searching ADRs in a `.decisions/` directory.
//! ADRs are indexed into an embedded HelixDB instance for fast semantic search.
//!
//! ## Features
//!
//! - **Semantic search**: Find ADRs by meaning, not just keywords
//! - **Persistent indexing**: HelixDB stores embeddings across invocations
//! - **Delta indexing**: Only re-index changed files
//! - **Metadata filtering**: Filter by status, tags
//!
//! ## Example
//!
//! ```no_run
//! use adr_search::{ADRSearcher, Status};
//! use std::path::Path;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut searcher = ADRSearcher::new()?;
//! searcher.sync(Path::new(".decisions/"))?;
//!
//! let results = searcher.search("database migration", 10, None, None)?;
//! for result in results.results {
//!     println!("{}: {} (score: {:.2})", result.id, result.title, result.score);
//! }
//! # Ok(())
//! # }
//! ```

pub mod delta;
pub mod embeddings;
pub mod loader;
pub mod searcher;
pub mod storage;
pub mod types;

pub use searcher::ADRSearcher;
pub use types::{ADR, SearchResponse, SearchResult, Status};
