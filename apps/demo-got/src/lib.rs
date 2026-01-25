//! Game of Thrones family tree graph demo with pluggable storage backends.
//!
//! This crate demonstrates graph database capabilities using the family
//! relationships from Game of Thrones. Supports both HelixDB and SurrealDB.

pub mod backend;
pub mod error;
pub mod loader;
pub mod query;
pub mod storage;
pub mod types;

pub use backend::{GotBackend, IngestStats};
pub use error::{GotError, Result};
pub use loader::{BioLoader, FamilyTree, PersonBio, RelationshipDef};
pub use query::{PersonFamily, find_ancestors, find_descendants, get_person_with_family};
pub use storage::{HelixDbBackend, SurrealDbBackend};
pub use types::{
    AncestorNode, DescendantNode, GraphStats, House, Person, RelationType, SearchResult,
};
