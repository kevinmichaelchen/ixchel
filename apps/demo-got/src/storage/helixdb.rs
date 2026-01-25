//! HelixDB storage backend for the family tree graph.

use crate::backend::{GotBackend, IngestStats};
use crate::error::{GotError, Result};
use crate::loader::{FamilyTree, RelationshipDef};
use crate::types::{GraphStats, House, Person, RelationType, SearchResult};
use bumpalo::Bump;
use helix_db::{
    helix_engine::{
        storage_core::{HelixGraphStorage, storage_methods::StorageMethods},
        traversal_core::config::{Config, GraphConfig, VectorConfig},
        types::SecondaryIndex,
        vector_core::hnsw::HNSW,
    },
    protocol::value::Value,
    utils::{items::Edge, label_hash::hash_label, properties::ImmutablePropertiesMap},
};
use ix_helixdb_ops as graph_ops;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

const NODE_LABEL: &str = "PERSON";

/// HelixDB storage backend for the Game of Thrones family tree.
pub struct HelixDbBackend {
    storage: HelixGraphStorage,
    db_path: PathBuf,
    /// Maps person ID (string) to node ID (u128).
    id_to_node: HashMap<String, u128>,
}

impl HelixDbBackend {
    /// Get the database path.
    #[must_use]
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    /// Internal: Insert a person as a node in the graph.
    fn insert_person_internal(&self, person: &Person) -> Result<u128> {
        let arena = Bump::new();
        let mut wtxn =
            self.storage.graph_env.write_txn().map_err(|e| {
                GotError::DatabaseError(format!("Failed to start transaction: {e}"))
            })?;

        let node_id = Uuid::new_v4().as_u128();
        let label: &str = arena.alloc_str(NODE_LABEL);

        let titles_json = serde_json::to_string(&person.titles).unwrap_or_default();
        let alias_str = person.alias.clone().unwrap_or_default();
        let is_alive_str = person.is_alive.to_string();

        let props: Vec<(&str, Value)> = vec![
            (arena.alloc_str("id"), Value::String(person.id.clone())),
            (arena.alloc_str("name"), Value::String(person.name.clone())),
            (
                arena.alloc_str("house"),
                Value::String(person.house.to_string()),
            ),
            (arena.alloc_str("titles"), Value::String(titles_json)),
            (arena.alloc_str("alias"), Value::String(alias_str)),
            (arena.alloc_str("is_alive"), Value::String(is_alive_str)),
        ];

        let properties = ImmutablePropertiesMap::new(props.len(), props.into_iter(), &arena);

        let node = helix_db::utils::items::Node {
            id: node_id,
            label,
            version: 1,
            properties: Some(properties),
        };

        graph_ops::put_node(&self.storage, &mut wtxn, &node)
            .map_err(|e| GotError::DatabaseError(format!("Failed to store node: {e}")))?;

        graph_ops::update_secondary_indices(&self.storage, &mut wtxn, &node).map_err(|e| {
            GotError::DatabaseError(format!("Failed to update secondary index: {e}"))
        })?;

        wtxn.commit()
            .map_err(|e| GotError::DatabaseError(format!("Failed to commit node: {e}")))?;

        Ok(node_id)
    }

    /// Internal: Create an edge between two nodes.
    fn create_edge_internal(
        &self,
        from_node_id: u128,
        to_node_id: u128,
        relation_type: RelationType,
    ) -> Result<()> {
        let arena = Bump::new();
        let mut wtxn =
            self.storage.graph_env.write_txn().map_err(|e| {
                GotError::DatabaseError(format!("Failed to start transaction: {e}"))
            })?;

        let edge_id = Uuid::new_v4().as_u128();
        let edge_label = arena.alloc_str(relation_type.as_edge_label());

        let edge = Edge {
            id: edge_id,
            label: edge_label,
            version: 1,
            from_node: from_node_id,
            to_node: to_node_id,
            properties: None,
        };

        graph_ops::put_edge(&self.storage, &mut wtxn, &edge)
            .map_err(|e| GotError::DatabaseError(format!("Failed to store edge: {e}")))?;

        wtxn.commit()
            .map_err(|e| GotError::DatabaseError(format!("Failed to commit edge: {e}")))?;

        Ok(())
    }

    /// Internal: Look up node ID by vector ID using the secondary index.
    fn lookup_by_vector_id(
        &self,
        rtxn: &heed3::RoTxn<'_>,
        vector_id: u128,
    ) -> Result<Option<u128>> {
        let key = Value::String(vector_id.to_string());
        graph_ops::lookup_secondary_index(&self.storage, rtxn, "vector_id", &key)
            .map_err(|e| GotError::DatabaseError(format!("Failed to lookup vector_id: {e}")))
    }

    /// Internal: Get a person from a node ID.
    fn get_person_internal(
        &self,
        rtxn: &heed3::RoTxn<'_>,
        node_id: u128,
        arena: &Bump,
    ) -> Result<Person> {
        let node = self
            .storage
            .get_node(rtxn, &node_id, arena)
            .map_err(|e| GotError::DatabaseError(format!("Failed to get node: {e:?}")))?;

        self.node_to_person(&node)
    }

    /// Convert a HelixDB node to a Person struct.
    fn node_to_person(&self, node: &helix_db::utils::items::Node<'_>) -> Result<Person> {
        let get_str = |name: &str| -> String {
            node.get_property(name)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .unwrap_or_default()
        };

        let get_bool = |name: &str| -> bool {
            node.get_property(name)
                .and_then(|v| match v {
                    Value::String(s) => s.parse().ok(),
                    _ => None,
                })
                .unwrap_or(false)
        };

        let id = get_str("id");
        let name = get_str("name");
        let house_str = get_str("house");
        let titles_json = get_str("titles");
        let alias_str = get_str("alias");
        let is_alive = get_bool("is_alive");

        let house: House = house_str
            .parse()
            .map_err(|e| GotError::DatabaseError(format!("Invalid house: {e}")))?;

        let titles: Vec<String> = serde_json::from_str(&titles_json).unwrap_or_default();
        let alias = if alias_str.is_empty() {
            None
        } else {
            Some(alias_str)
        };

        Ok(Person {
            id,
            name,
            house,
            titles,
            alias,
            is_alive,
        })
    }

    /// Parse a string node ID back to u128.
    fn parse_node_id(node_id: &str) -> Result<u128> {
        node_id
            .parse()
            .map_err(|e| GotError::DatabaseError(format!("Invalid node ID '{node_id}': {e}")))
    }
}

impl GotBackend for HelixDbBackend {
    fn new(db_path: &Path) -> Result<Self> {
        let graph_path = db_path.join("graph.db");
        std::fs::create_dir_all(&graph_path).map_err(|e| {
            GotError::DatabaseError(format!("Failed to create database directory: {e}"))
        })?;

        let config = Config {
            vector_config: Some(VectorConfig {
                m: Some(16),
                ef_construction: Some(128),
                ef_search: Some(64),
            }),
            graph_config: Some(GraphConfig {
                secondary_indices: Some(vec![
                    SecondaryIndex::Index("id".to_string()),
                    SecondaryIndex::Index("house".to_string()),
                    SecondaryIndex::Index("vector_id".to_string()),
                ]),
            }),
            db_max_size_gb: Some(1),
            ..Default::default()
        };

        let version_info =
            helix_db::helix_engine::storage_core::version_info::VersionInfo::default();

        let storage =
            HelixGraphStorage::new(&graph_path.to_string_lossy(), config, version_info)
                .map_err(|e| GotError::DatabaseError(format!("Failed to create storage: {e:?}")))?;

        Ok(Self {
            storage,
            db_path: db_path.to_path_buf(),
            id_to_node: HashMap::new(),
        })
    }

    fn exists(db_path: &Path) -> bool {
        db_path.join("graph.db").exists()
    }

    fn clear(&self) -> Result<()> {
        let mut wtxn =
            self.storage.graph_env.write_txn().map_err(|e| {
                GotError::DatabaseError(format!("Failed to start transaction: {e}"))
            })?;

        self.storage
            .nodes_db
            .clear(&mut wtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to clear nodes: {e}")))?;

        self.storage
            .edges_db
            .clear(&mut wtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to clear edges: {e}")))?;

        self.storage
            .out_edges_db
            .clear(&mut wtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to clear out_edges: {e}")))?;

        self.storage
            .in_edges_db
            .clear(&mut wtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to clear in_edges: {e}")))?;

        for (index_name, (db, _)) in &self.storage.secondary_indices {
            db.clear(&mut wtxn).map_err(|e| {
                GotError::DatabaseError(format!(
                    "Failed to clear secondary index {index_name}: {e}"
                ))
            })?;
        }

        self.storage
            .vectors
            .vectors_db
            .clear(&mut wtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to clear vectors: {e}")))?;

        self.storage
            .vectors
            .vector_properties_db
            .clear(&mut wtxn)
            .map_err(|e| {
                GotError::DatabaseError(format!("Failed to clear vector properties: {e}"))
            })?;

        self.storage
            .vectors
            .edges_db
            .clear(&mut wtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to clear vector edges: {e}")))?;

        wtxn.commit()
            .map_err(|e| GotError::DatabaseError(format!("Failed to commit clear: {e}")))?;

        Ok(())
    }

    fn ingest(&mut self, tree: &FamilyTree) -> Result<IngestStats> {
        let mut stats = IngestStats::default();

        // First pass: insert all people as nodes
        for person in &tree.people {
            let node_id = self.insert_person_internal(person)?;
            self.id_to_node.insert(person.id.clone(), node_id);
            stats.nodes_inserted += 1;
        }

        // Second pass: create all relationship edges
        for rel in &tree.relationships {
            match rel {
                RelationshipDef::ParentOf { from, to } => {
                    let from_node = self
                        .id_to_node
                        .get(from)
                        .copied()
                        .ok_or_else(|| GotError::PersonNotFound(from.clone()))?;

                    for child_id in to {
                        let to_node = self
                            .id_to_node
                            .get(child_id)
                            .copied()
                            .ok_or_else(|| GotError::PersonNotFound(child_id.clone()))?;
                        self.create_edge_internal(from_node, to_node, RelationType::ParentOf)?;
                        stats.edges_inserted += 1;
                    }
                }
                RelationshipDef::SpouseOf { between } => {
                    if between.len() >= 2 {
                        let a = self
                            .id_to_node
                            .get(&between[0])
                            .copied()
                            .ok_or_else(|| GotError::PersonNotFound(between[0].clone()))?;
                        let b = self
                            .id_to_node
                            .get(&between[1])
                            .copied()
                            .ok_or_else(|| GotError::PersonNotFound(between[1].clone()))?;
                        // Bidirectional: create edges in both directions
                        self.create_edge_internal(a, b, RelationType::SpouseOf)?;
                        self.create_edge_internal(b, a, RelationType::SpouseOf)?;
                        stats.edges_inserted += 2;
                    }
                }
                RelationshipDef::SiblingOf { between } => {
                    // Create edges between all pairs (bidirectional)
                    for i in 0..between.len() {
                        for j in (i + 1)..between.len() {
                            let a = self
                                .id_to_node
                                .get(&between[i])
                                .copied()
                                .ok_or_else(|| GotError::PersonNotFound(between[i].clone()))?;
                            let b = self
                                .id_to_node
                                .get(&between[j])
                                .copied()
                                .ok_or_else(|| GotError::PersonNotFound(between[j].clone()))?;
                            self.create_edge_internal(a, b, RelationType::SiblingOf)?;
                            self.create_edge_internal(b, a, RelationType::SiblingOf)?;
                            stats.edges_inserted += 2;
                        }
                    }
                }
            }
        }

        Ok(stats)
    }

    fn insert_person_basic(&self, person: &Person) -> Result<String> {
        let node_id = self.insert_person_internal(person)?;
        Ok(node_id.to_string())
    }

    fn insert_person_with_embedding(
        &self,
        person: &Person,
        embedding: &[f32],
    ) -> Result<(String, String)> {
        let arena = Bump::new();
        let mut wtxn =
            self.storage.graph_env.write_txn().map_err(|e| {
                GotError::DatabaseError(format!("Failed to start transaction: {e}"))
            })?;

        // Insert the vector first
        let embedding_f64: Vec<f64> = embedding.iter().map(|&x| f64::from(x)).collect();
        let label: &str = arena.alloc_str(NODE_LABEL);

        let vector =
            self.storage
                .vectors
                .insert::<fn(
                    &helix_db::helix_engine::vector_core::vector::HVector<'_>,
                    &heed3::RoTxn<'_>,
                ) -> bool>(&mut wtxn, label, &embedding_f64, None, &arena)
                .map_err(|e| GotError::EmbeddingError(format!("Failed to insert vector: {e:?}")))?;

        let vector_id = vector.id;
        let node_id = Uuid::new_v4().as_u128();

        let titles_json = serde_json::to_string(&person.titles).unwrap_or_default();
        let alias_str = person.alias.clone().unwrap_or_default();
        let is_alive_str = person.is_alive.to_string();

        let props: Vec<(&str, Value)> = vec![
            (arena.alloc_str("id"), Value::String(person.id.clone())),
            (arena.alloc_str("name"), Value::String(person.name.clone())),
            (
                arena.alloc_str("house"),
                Value::String(person.house.to_string()),
            ),
            (arena.alloc_str("titles"), Value::String(titles_json)),
            (arena.alloc_str("alias"), Value::String(alias_str)),
            (arena.alloc_str("is_alive"), Value::String(is_alive_str)),
            (
                arena.alloc_str("vector_id"),
                Value::String(vector_id.to_string()),
            ),
        ];

        let properties = ImmutablePropertiesMap::new(props.len(), props.into_iter(), &arena);

        let node = helix_db::utils::items::Node {
            id: node_id,
            label,
            version: 1,
            properties: Some(properties),
        };

        graph_ops::put_node(&self.storage, &mut wtxn, &node)
            .map_err(|e| GotError::DatabaseError(format!("Failed to store node: {e}")))?;

        graph_ops::update_secondary_indices(&self.storage, &mut wtxn, &node).map_err(|e| {
            GotError::DatabaseError(format!("Failed to update secondary index: {e}"))
        })?;

        wtxn.commit()
            .map_err(|e| GotError::DatabaseError(format!("Failed to commit node: {e}")))?;

        Ok((node_id.to_string(), vector_id.to_string()))
    }

    fn create_edge(
        &self,
        from_node_id: &str,
        to_node_id: &str,
        relation_type: RelationType,
    ) -> Result<()> {
        let from_id = Self::parse_node_id(from_node_id)?;
        let to_id = Self::parse_node_id(to_node_id)?;
        self.create_edge_internal(from_id, to_id, relation_type)
    }

    fn search_semantic(&self, embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>> {
        let arena = Bump::new();
        let rtxn = self.storage.graph_env.read_txn().map_err(|e| {
            GotError::DatabaseError(format!("Failed to start read transaction: {e}"))
        })?;

        let query_f64: Vec<f64> = embedding.iter().map(|&x| f64::from(x)).collect();
        let label: &str = arena.alloc_str(NODE_LABEL);

        let vector_results =
            self.storage
                .vectors
                .search::<fn(
                    &helix_db::helix_engine::vector_core::vector::HVector<'_>,
                    &heed3::RoTxn<'_>,
                ) -> bool>(&rtxn, &query_f64, limit, label, None, false, &arena)
                .map_err(|e| GotError::VectorSearchError(format!("Vector search failed: {e:?}")))?;

        let mut results = Vec::new();

        for hvector in vector_results {
            let vector_id = hvector.id;
            let distance = hvector.get_distance() as f32;
            // Convert distance to similarity: closer distance = higher similarity
            let score = 1.0 / (1.0 + distance);

            // Look up the node by vector_id
            if let Some(node_id) = self.lookup_by_vector_id(&rtxn, vector_id)?
                && let Ok(person) = self.get_person_internal(&rtxn, node_id, &arena)
            {
                results.push(SearchResult {
                    person,
                    score,
                    snippet: None,
                });
            }
        }

        Ok(results)
    }

    fn lookup_by_id(&self, person_id: &str) -> Result<Option<String>> {
        let rtxn = self.storage.graph_env.read_txn().map_err(|e| {
            GotError::DatabaseError(format!("Failed to start read transaction: {e}"))
        })?;

        let key = Value::String(person_id.to_string());
        if let Some(node_id) =
            graph_ops::lookup_secondary_index(&self.storage, &rtxn, "id", &key)
                .map_err(|e| GotError::DatabaseError(format!("Failed to lookup: {e}")))?
        {
            return Ok(Some(node_id.to_string()));
        }

        Ok(None)
    }

    fn get_person(&self, node_id: &str) -> Result<Person> {
        let arena = Bump::new();
        let rtxn = self.storage.graph_env.read_txn().map_err(|e| {
            GotError::DatabaseError(format!("Failed to start read transaction: {e}"))
        })?;

        let node_id_u128 = Self::parse_node_id(node_id)?;
        let node = self
            .storage
            .get_node(&rtxn, &node_id_u128, &arena)
            .map_err(|e| GotError::DatabaseError(format!("Failed to get node: {e:?}")))?;

        self.node_to_person(&node)
    }

    fn get_incoming_neighbors(
        &self,
        node_id: &str,
        relation_type: RelationType,
    ) -> Result<Vec<String>> {
        let rtxn = self.storage.graph_env.read_txn().map_err(|e| {
            GotError::DatabaseError(format!("Failed to start read transaction: {e}"))
        })?;

        let node_id_u128 = Self::parse_node_id(node_id)?;
        let label_hash = hash_label(relation_type.as_edge_label(), None);
        let neighbors =
            graph_ops::incoming_neighbors(&self.storage, &rtxn, node_id_u128, &label_hash)
                .map_err(|e| {
                    GotError::DatabaseError(format!("Failed to read incoming edges: {e}"))
                })?;

        Ok(neighbors.into_iter().map(|id| id.to_string()).collect())
    }

    fn get_outgoing_neighbors(
        &self,
        node_id: &str,
        relation_type: RelationType,
    ) -> Result<Vec<String>> {
        let rtxn = self.storage.graph_env.read_txn().map_err(|e| {
            GotError::DatabaseError(format!("Failed to start read transaction: {e}"))
        })?;

        let node_id_u128 = Self::parse_node_id(node_id)?;
        let label_hash = hash_label(relation_type.as_edge_label(), None);
        let neighbors =
            graph_ops::outgoing_neighbors(&self.storage, &rtxn, node_id_u128, &label_hash)
                .map_err(|e| {
                    GotError::DatabaseError(format!("Failed to read outgoing edges: {e}"))
                })?;

        Ok(neighbors.into_iter().map(|id| id.to_string()).collect())
    }

    fn get_stats(&self) -> Result<GraphStats> {
        let rtxn = self.storage.graph_env.read_txn().map_err(|e| {
            GotError::DatabaseError(format!("Failed to start read transaction: {e}"))
        })?;

        let arena = Bump::new();
        let mut node_count = 0;
        let mut edge_count = 0;
        let mut house_counts: HashMap<String, usize> = HashMap::new();

        // Count nodes and collect house statistics
        let iter = self
            .storage
            .nodes_db
            .iter(&rtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to iterate nodes: {e}")))?;

        for result in iter {
            let (node_id, value) =
                result.map_err(|e| GotError::DatabaseError(format!("Failed to read node: {e}")))?;

            if let Ok(node) =
                helix_db::utils::items::Node::from_bincode_bytes(node_id, value, &arena)
            {
                node_count += 1;
                if let Some(Value::String(house)) = node.get_property("house") {
                    *house_counts.entry(house.clone()).or_insert(0) += 1;
                }
            }
        }

        // Count edges
        let edge_iter = self
            .storage
            .edges_db
            .iter(&rtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to iterate edges: {e}")))?;

        for result in edge_iter {
            if result.is_ok() {
                edge_count += 1;
            }
        }

        Ok(GraphStats {
            node_count,
            edge_count,
            house_counts,
        })
    }

    fn get_house_members(&self, house: House) -> Result<Vec<Person>> {
        let rtxn = self.storage.graph_env.read_txn().map_err(|e| {
            GotError::DatabaseError(format!("Failed to start read transaction: {e}"))
        })?;

        let arena = Bump::new();
        let house_str = house.to_string();
        let mut members = Vec::new();

        let iter = self
            .storage
            .nodes_db
            .iter(&rtxn)
            .map_err(|e| GotError::DatabaseError(format!("Failed to iterate nodes: {e}")))?;

        for result in iter {
            let (node_id, value) =
                result.map_err(|e| GotError::DatabaseError(format!("Failed to read node: {e}")))?;

            if let Ok(node) =
                helix_db::utils::items::Node::from_bincode_bytes(node_id, value, &arena)
                && let Some(Value::String(node_house)) = node.get_property("house")
                && node_house == &house_str
                && let Ok(person) = self.node_to_person(&node)
            {
                members.push(person);
            }
        }

        Ok(members)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn build_tree() -> FamilyTree {
        FamilyTree {
            houses: Vec::new(),
            people: vec![
                Person {
                    id: "ned-stark".to_string(),
                    name: "Eddard Stark".to_string(),
                    house: House::Stark,
                    titles: vec!["Lord of Winterfell".to_string()],
                    alias: None,
                    is_alive: true,
                },
                Person {
                    id: "catelyn-stark".to_string(),
                    name: "Catelyn Stark".to_string(),
                    house: House::Tully,
                    titles: Vec::new(),
                    alias: None,
                    is_alive: true,
                },
                Person {
                    id: "robb-stark".to_string(),
                    name: "Robb Stark".to_string(),
                    house: House::Stark,
                    titles: vec!["King in the North".to_string()],
                    alias: None,
                    is_alive: true,
                },
            ],
            relationships: vec![
                RelationshipDef::ParentOf {
                    from: "ned-stark".to_string(),
                    to: vec!["robb-stark".to_string()],
                },
                RelationshipDef::SpouseOf {
                    between: vec!["ned-stark".to_string(), "catelyn-stark".to_string()],
                },
            ],
        }
    }

    fn open_storage() -> Result<(TempDir, HelixDbBackend)> {
        let temp = TempDir::new()?;
        let storage = HelixDbBackend::new(temp.path())?;
        Ok((temp, storage))
    }

    #[test]
    fn test_ingest_and_relationship_queries() -> Result<()> {
        let (_temp, mut storage) = open_storage()?;
        let tree = build_tree();
        let stats = storage.ingest(&tree)?;

        assert_eq!(stats.nodes_inserted, 3);
        assert_eq!(stats.edges_inserted, 3);

        let ned_node = storage.lookup_by_id("ned-stark")?.expect("ned node");
        let robb_node = storage.lookup_by_id("robb-stark")?.expect("robb node");
        let catelyn_node = storage
            .lookup_by_id("catelyn-stark")?
            .expect("catelyn node");

        let ned = storage.get_person(&ned_node)?;
        assert_eq!(ned.name, "Eddard Stark");
        assert_eq!(ned.house, House::Stark);

        let outgoing_parent = storage.get_outgoing_neighbors(&ned_node, RelationType::ParentOf)?;
        assert_eq!(outgoing_parent, vec![robb_node.clone()]);

        let incoming_parent = storage.get_incoming_neighbors(&robb_node, RelationType::ParentOf)?;
        assert_eq!(incoming_parent, vec![ned_node.clone()]);

        let spouse_out = storage.get_outgoing_neighbors(&ned_node, RelationType::SpouseOf)?;
        assert_eq!(spouse_out, vec![catelyn_node.clone()]);

        let spouse_in = storage.get_incoming_neighbors(&ned_node, RelationType::SpouseOf)?;
        assert_eq!(spouse_in, vec![catelyn_node]);

        Ok(())
    }

    #[test]
    fn test_stats_house_members_and_clear() -> Result<()> {
        let (_temp, mut storage) = open_storage()?;
        let tree = build_tree();
        storage.ingest(&tree)?;

        let stats = storage.get_stats()?;
        assert_eq!(stats.node_count, 3);
        assert_eq!(stats.edge_count, 3);
        assert_eq!(stats.house_counts.get("Stark").copied().unwrap_or(0), 2);
        assert_eq!(stats.house_counts.get("Tully").copied().unwrap_or(0), 1);

        let stark_members = storage.get_house_members(House::Stark)?;
        let mut stark_ids: Vec<String> = stark_members.into_iter().map(|p| p.id).collect();
        stark_ids.sort();
        assert_eq!(
            stark_ids,
            vec!["ned-stark".to_string(), "robb-stark".to_string()]
        );

        storage.clear()?;
        let cleared_stats = storage.get_stats()?;
        assert_eq!(cleared_stats.node_count, 0);
        assert_eq!(cleared_stats.edge_count, 0);
        assert!(cleared_stats.house_counts.is_empty());

        Ok(())
    }
}
