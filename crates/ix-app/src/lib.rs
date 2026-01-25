//! Ixchel application wiring layer.
//!
//! This crate provides the high-level API for interacting with Ixchel storage backends.
//! Currently only `SurrealDB` is supported in the published crate.
//!
//! # `HelixDB` Support
//!
//! `HelixDB` backend is not available from crates.io because `helix-db` uses a git
//! dependency. If you need `HelixDB` support, build from source:
//! <https://github.com/kevinmichaelchen/ixchel>

use anyhow::{Context, Result};
use ix_core::index::{IndexBackend, SearchHit, SyncStats};
use ix_core::repo::IxchelRepo;

fn backend_name(repo: &IxchelRepo) -> String {
    repo.config.storage.backend.trim().to_ascii_lowercase()
}

pub fn sync(repo: &IxchelRepo) -> Result<SyncStats> {
    match backend_name(repo).as_str() {
        "surrealdb" => {
            let mut index =
                ix_storage_surrealdb::SurrealDbIndex::open(repo).context("open surrealdb index")?;
            IndexBackend::sync(&mut index, repo).context("sync surrealdb index")
        }
        "helixdb" => anyhow::bail!(
            "HelixDB backend is not available in this build. \
             The crates.io version only supports SurrealDB because helix-db \
             uses a git dependency. Build from source for HelixDB support: \
             https://github.com/kevinmichaelchen/ixchel"
        ),
        backend => anyhow::bail!("Unsupported storage backend: {backend}"),
    }
}

pub fn search(repo: &IxchelRepo, query: &str, limit: usize) -> Result<Vec<SearchHit>> {
    match backend_name(repo).as_str() {
        "surrealdb" => {
            let index =
                ix_storage_surrealdb::SurrealDbIndex::open(repo).context("open surrealdb index")?;
            IndexBackend::search(&index, query, limit).context("search surrealdb index")
        }
        "helixdb" => anyhow::bail!(
            "HelixDB backend is not available in this build. \
             The crates.io version only supports SurrealDB because helix-db \
             uses a git dependency. Build from source for HelixDB support: \
             https://github.com/kevinmichaelchen/ixchel"
        ),
        backend => anyhow::bail!("Unsupported storage backend: {backend}"),
    }
}

pub fn health_check(repo: &IxchelRepo) -> Result<()> {
    match backend_name(repo).as_str() {
        "surrealdb" => {
            let index =
                ix_storage_surrealdb::SurrealDbIndex::open(repo).context("open surrealdb index")?;
            IndexBackend::health_check(&index).context("surrealdb health check")
        }
        "helixdb" => anyhow::bail!(
            "HelixDB backend is not available in this build. \
             The crates.io version only supports SurrealDB because helix-db \
             uses a git dependency. Build from source for HelixDB support: \
             https://github.com/kevinmichaelchen/ixchel"
        ),
        backend => anyhow::bail!("Unsupported storage backend: {backend}"),
    }
}
