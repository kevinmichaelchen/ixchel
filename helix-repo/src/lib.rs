//! helix-repo: Manage git repository clones for helix-tools
//!
//! This crate provides both a CLI and library API for cloning git repositories
//! to a standardized directory structure organized by domain/owner/repo.
//!
//! # Example
//!
//! ```ignore
//! use helix_repo::RepositoryManager;
//!
//! let manager = RepositoryManager::from_config()?;
//! let path = manager.clone("https://github.com/facebook/react")?;
//! ```

pub mod error;

pub use error::Error;

/// Result type for helix-repo operations
pub type Result<T> = std::result::Result<T, Error>;
