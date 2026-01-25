//! Storage backend trait for pluggable graph storage.

use crate::error::Result;
use crate::loader::FamilyTree;
use crate::types::{GraphStats, House, Person, RelationType, SearchResult};
use std::path::Path;

/// Statistics from an ingest operation.
#[derive(Debug, Default)]
pub struct IngestStats {
    pub nodes_inserted: usize,
    pub edges_inserted: usize,
}

/// Abstract storage backend for the Game of Thrones family tree.
///
/// Implementations provide graph storage with vector similarity search.
pub trait GotBackend: Send + Sync {
    /// Create or open a storage instance at the given path.
    fn new(db_path: &Path) -> Result<Self>
    where
        Self: Sized;

    /// Check if the database exists and has data.
    fn exists(db_path: &Path) -> bool
    where
        Self: Sized;

    /// Clear all data from the database.
    fn clear(&self) -> Result<()>;

    /// Ingest a family tree into the database.
    fn ingest(&mut self, tree: &FamilyTree) -> Result<IngestStats>;

    /// Insert a person as a node without an embedding.
    fn insert_person_basic(&self, person: &Person) -> Result<String>;

    /// Insert a person as a node with an embedding vector.
    /// Returns (node_id, vector_id).
    fn insert_person_with_embedding(
        &self,
        person: &Person,
        embedding: &[f32],
    ) -> Result<(String, String)>;

    /// Create an edge between two nodes.
    fn create_edge(
        &self,
        from_node_id: &str,
        to_node_id: &str,
        relation_type: RelationType,
    ) -> Result<()>;

    /// Perform semantic search using a query embedding.
    fn search_semantic(&self, embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>>;

    /// Look up a node ID by person ID.
    fn lookup_by_id(&self, person_id: &str) -> Result<Option<String>>;

    /// Get a person from a node ID.
    fn get_person(&self, node_id: &str) -> Result<Person>;

    /// Get all nodes connected by incoming edges of a specific type.
    fn get_incoming_neighbors(
        &self,
        node_id: &str,
        relation_type: RelationType,
    ) -> Result<Vec<String>>;

    /// Get all nodes connected by outgoing edges of a specific type.
    fn get_outgoing_neighbors(
        &self,
        node_id: &str,
        relation_type: RelationType,
    ) -> Result<Vec<String>>;

    /// Get statistics about the graph.
    fn get_stats(&self) -> Result<GraphStats>;

    /// Get all people belonging to a specific house.
    fn get_house_members(&self, house: House) -> Result<Vec<Person>>;
}
