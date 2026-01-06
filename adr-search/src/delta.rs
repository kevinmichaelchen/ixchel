//! Delta detection for incremental indexing.

use crate::types::ADR;
use std::collections::{HashMap, HashSet};

/// Result of delta computation.
pub struct DeltaResult {
    /// ADRs that need to be added or updated.
    pub to_add: Vec<ADR>,
    /// File paths of ADRs that need to be removed.
    pub to_remove: Vec<String>,
}

/// Compute delta between filesystem and indexed ADRs.
///
/// Compares current ADRs against stored hashes to determine
/// which ADRs need to be re-indexed and which need to be removed.
pub fn compute_delta(
    current_adrs: Vec<ADR>,
    stored_hashes: HashMap<String, String>,
) -> DeltaResult {
    let mut to_add = Vec::new();
    let mut to_remove = Vec::new();

    // Track which stored paths we've seen
    let mut seen_paths: HashSet<String> = HashSet::new();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ADRMetadata, Status};
    use chrono::NaiveDate;
    use std::path::PathBuf;

    fn make_adr(id: u32, path: &str, hash: &str) -> ADR {
        ADR {
            metadata: ADRMetadata {
                id,
                title: format!("ADR {id}"),
                status: Status::Accepted,
                date: NaiveDate::from_ymd_opt(2026, 1, 5).unwrap(),
                deciders: vec![],
                tags: vec![],
                supersedes: None,
                superseded_by: None,
            },
            body: String::new(),
            file_path: PathBuf::from(path),
            content_hash: hash.to_string(),
            embedding: None,
        }
    }

    #[test]
    fn test_no_changes() {
        let current = vec![make_adr(1, "001.md", "hash1")];
        let stored: HashMap<_, _> = [("001.md".to_string(), "hash1".to_string())]
            .into_iter()
            .collect();

        let delta = compute_delta(current, stored);
        assert!(delta.to_add.is_empty());
        assert!(delta.to_remove.is_empty());
    }

    #[test]
    fn test_new_adr() {
        let current = vec![make_adr(1, "001.md", "hash1")];
        let stored: HashMap<String, String> = HashMap::new();

        let delta = compute_delta(current, stored);
        assert_eq!(delta.to_add.len(), 1);
        assert!(delta.to_remove.is_empty());
    }

    #[test]
    fn test_changed_adr() {
        let current = vec![make_adr(1, "001.md", "hash2")];
        let stored: HashMap<_, _> = [("001.md".to_string(), "hash1".to_string())]
            .into_iter()
            .collect();

        let delta = compute_delta(current, stored);
        assert_eq!(delta.to_add.len(), 1);
        assert!(delta.to_remove.is_empty());
    }

    #[test]
    fn test_deleted_adr() {
        let current = vec![];
        let stored: HashMap<_, _> = [("001.md".to_string(), "hash1".to_string())]
            .into_iter()
            .collect();

        let delta = compute_delta(current, stored);
        assert!(delta.to_add.is_empty());
        assert_eq!(delta.to_remove.len(), 1);
    }
}
