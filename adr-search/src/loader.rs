//! Load ADRs from filesystem.

use crate::types::{ADR, ADRMetadata};
use anyhow::{Context, Result};
use gray_matter::{Matter, engine::YAML};
use sha2::{Digest, Sha256};
use std::path::Path;

/// Load all ADRs from a directory.
///
/// Scans the directory for `.md` files, parses YAML frontmatter,
/// and returns a list of ADRs. Malformed files are skipped with a warning.
pub fn load_adrs(dir: &Path) -> Result<Vec<ADR>> {
    let mut adrs = Vec::new();
    let matter = Matter::<YAML>::new();

    if !dir.exists() {
        anyhow::bail!("Directory not found: {}", dir.display());
    }

    for entry in std::fs::read_dir(dir).context("Failed to read directory")? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|e| e == "md") {
            match load_adr(&path, &matter) {
                Ok(adr) => adrs.push(adr),
                Err(e) => eprintln!("Warning: Skipping {}: {e}", path.display()),
            }
        }
    }

    Ok(adrs)
}

/// Load a single ADR from a file.
fn load_adr(path: &Path, matter: &Matter<YAML>) -> Result<ADR> {
    let content = std::fs::read_to_string(path).context("Failed to read file")?;

    let parsed = matter.parse(&content);

    let metadata: ADRMetadata = parsed
        .data
        .ok_or_else(|| anyhow::anyhow!("Missing YAML frontmatter"))?
        .deserialize()
        .context("Failed to parse frontmatter")?;

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

/// Compute SHA256 hash of content.
fn hash_content(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_adr(dir: &Path, filename: &str, content: &str) {
        let path = dir.join(filename);
        let mut file = std::fs::File::create(path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_load_valid_adr() {
        let dir = TempDir::new().unwrap();
        create_test_adr(
            dir.path(),
            "001-test.md",
            r#"---
id: 1
title: Test ADR
status: accepted
date: 2026-01-05
deciders:
  - Alice
tags:
  - testing
---

# Context

This is a test ADR.
"#,
        );

        let adrs = load_adrs(dir.path()).unwrap();
        assert_eq!(adrs.len(), 1);
        assert_eq!(adrs[0].metadata.id, 1);
        assert_eq!(adrs[0].metadata.title, "Test ADR");
    }

    #[test]
    fn test_skip_malformed_adr() {
        let dir = TempDir::new().unwrap();
        create_test_adr(dir.path(), "001-bad.md", "No frontmatter here");

        let adrs = load_adrs(dir.path()).unwrap();
        assert_eq!(adrs.len(), 0);
    }

    #[test]
    fn test_missing_directory() {
        let result = load_adrs(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }
}
