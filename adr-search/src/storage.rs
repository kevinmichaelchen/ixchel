//! Storage abstraction for ADR indexing.

use crate::types::ADR;
use anyhow::Result;
use std::collections::HashMap;

/// Storage trait for ADR indexing.
///
/// Implementations should persist embeddings and metadata
/// for fast retrieval across invocations.
pub trait ADRStorage: Send + Sync {
    /// Store ADRs with embeddings.
    fn index(&mut self, adrs: Vec<ADR>) -> Result<()>;

    /// Remove ADRs by file path.
    fn remove(&mut self, paths: Vec<String>) -> Result<()>;

    /// Search by embedding similarity.
    ///
    /// Returns ADRs with similarity scores, sorted by score descending.
    fn search(&self, embedding: Vec<f32>, limit: usize) -> Result<Vec<(ADR, f32)>>;

    /// Get stored content hashes for delta detection.
    ///
    /// Returns a map of file path -> content hash.
    fn get_hashes(&self) -> Result<HashMap<String, String>>;
}

/// Embedded HelixDB storage.
///
/// Stores ADR embeddings and metadata in an embedded HelixDB instance
/// at `~/.helix/data/adr/`.
pub struct HelixDBStorage {
    // TODO: Add HelixDB client when available
    // db: HelixDB,
    //
    // For now, use in-memory storage as placeholder
    adrs: Vec<ADR>,
    hashes: HashMap<String, String>,
}

impl HelixDBStorage {
    /// Open or create the HelixDB storage.
    pub fn open() -> Result<Self> {
        // TODO: Initialize embedded HelixDB
        // let db_path = helix_config::helix_data_dir()?.join("adr");
        // let db = HelixDB::open(&db_path)?;

        Ok(Self {
            adrs: Vec::new(),
            hashes: HashMap::new(),
        })
    }
}

impl ADRStorage for HelixDBStorage {
    fn index(&mut self, adrs: Vec<ADR>) -> Result<()> {
        for adr in adrs {
            let path = adr.file_path.to_string_lossy().to_string();
            self.hashes.insert(path, adr.content_hash.clone());
            self.adrs.push(adr);
        }
        Ok(())

        // TODO: Implement HelixDB indexing
        // for adr in adrs {
        //     let doc = Document {
        //         id: format!("adr/{}", adr.metadata.id),
        //         content: adr.body.clone(),
        //         metadata: serde_json::to_value(&adr.metadata)?,
        //         embedding: adr.embedding.clone().unwrap_or_default(),
        //     };
        //     self.db.upsert(doc)?;
        // }
        // Ok(())
    }

    fn remove(&mut self, paths: Vec<String>) -> Result<()> {
        for path in &paths {
            self.hashes.remove(path);
            self.adrs.retain(|a| a.file_path.to_string_lossy() != *path);
        }
        Ok(())

        // TODO: Implement HelixDB removal
        // for path in paths {
        //     self.db.delete(&path)?;
        // }
        // Ok(())
    }

    fn search(&self, embedding: Vec<f32>, limit: usize) -> Result<Vec<(ADR, f32)>> {
        // Simple cosine similarity search (placeholder for HelixDB)
        let mut results: Vec<(ADR, f32)> = self
            .adrs
            .iter()
            .filter_map(|adr| {
                adr.embedding.as_ref().map(|emb| {
                    let score = cosine_similarity(&embedding, emb);
                    (adr.clone(), score)
                })
            })
            .collect();

        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(limit);

        Ok(results)

        // TODO: Implement HelixDB vector search
        // self.db.vector_search(&embedding, limit)
    }

    fn get_hashes(&self) -> Result<HashMap<String, String>> {
        Ok(self.hashes.clone())

        // TODO: Implement HelixDB hash retrieval
        // self.db.get_all_hashes()
    }
}

/// Compute cosine similarity between two vectors.
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot / (norm_a * norm_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&a, &c)).abs() < 0.001);
    }
}
