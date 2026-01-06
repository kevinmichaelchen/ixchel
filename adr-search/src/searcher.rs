//! Main search logic.

use crate::delta::compute_delta;
use crate::embeddings::Embedder;
use crate::loader::load_adrs;
use crate::storage::{ADRStorage, HelixDBStorage};
use crate::types::{SearchResponse, SearchResult, Status};
use anyhow::Result;
use std::path::Path;

/// ADR searcher with embedded storage and embeddings.
pub struct ADRSearcher {
    storage: Box<dyn ADRStorage>,
    embedder: Embedder,
}

impl ADRSearcher {
    /// Create a new searcher with default HelixDB storage.
    pub fn new() -> Result<Self> {
        let storage = Box::new(HelixDBStorage::open()?);
        let embedder = Embedder::new()?;
        Ok(Self { storage, embedder })
    }

    /// Sync ADRs from directory into storage.
    ///
    /// Performs delta indexing: only re-indexes changed files,
    /// removes deleted files from the index.
    pub fn sync(&mut self, dir: &Path) -> Result<()> {
        // Load current ADRs from filesystem
        let adrs = load_adrs(dir)?;

        // Get stored hashes for delta detection
        let stored_hashes = self.storage.get_hashes()?;

        // Compute what needs to be updated
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

    /// Search for ADRs matching query.
    ///
    /// Returns ranked results filtered by optional status and tags.
    pub fn search(
        &self,
        query: &str,
        limit: usize,
        status_filter: Option<Status>,
        tags_filter: Option<Vec<String>>,
    ) -> Result<SearchResponse> {
        // Embed query
        let query_embedding = self.embedder.embed(query)?;

        // Search storage (over-fetch for filtering)
        let results = self.storage.search(query_embedding, limit * 2)?;

        // Filter and convert
        let search_results: Vec<SearchResult> = results
            .into_iter()
            .filter(|(adr, _)| {
                // Status filter
                if let Some(ref status) = status_filter
                    && &adr.metadata.status != status
                {
                    return false;
                }
                // Tags filter (all tags must match)
                if let Some(ref tags) = tags_filter
                    && !tags.iter().all(|t| adr.metadata.tags.contains(t))
                {
                    return false;
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

        Ok(SearchResponse {
            query: query.to_string(),
            count: search_results.len(),
            results: search_results,
        })
    }
}
