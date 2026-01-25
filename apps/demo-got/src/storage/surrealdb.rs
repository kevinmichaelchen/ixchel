//! SurrealDB storage backend for the family tree graph.

use crate::backend::{GotBackend, IngestStats};
use crate::error::{GotError, Result};
use crate::loader::{FamilyTree, RelationshipDef};
use crate::types::{GraphStats, House, Person, RelationType, SearchResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, SurrealKv};
use tokio::runtime::Runtime;

/// SurrealDB storage backend for the Game of Thrones family tree.
pub struct SurrealDbBackend {
    db: Option<Surreal<Db>>,
    db_path: PathBuf,
    runtime: Arc<Runtime>,
    /// Maps person ID to record key for relationship creation.
    id_to_record: HashMap<String, String>,
}

/// Person record stored in SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersonRecord {
    person_id: String,
    name: String,
    house: String,
    titles: Vec<String>,
    alias: Option<String>,
    is_alive: bool,
    #[serde(default)]
    embedding: Vec<f32>,
}

/// Search result from vector similarity query.
#[derive(Debug, Clone, Deserialize)]
struct VectorSearchResult {
    person_id: String,
    name: String,
    house: String,
    titles: String,
    alias: String,
    is_alive: String,
    distance: f64,
}

/// Result of a neighbor query.
#[derive(Debug, Clone, Deserialize)]
struct NeighborResult {
    person_id: String,
}

/// Result of a count query.
#[derive(Debug, Clone, Deserialize)]
struct CountResult {
    count: usize,
}

/// House count result.
#[derive(Debug, Clone, Deserialize)]
struct HouseCountResult {
    house: String,
    count: usize,
}

impl SurrealDbBackend {
    /// Get the database path.
    #[must_use]
    pub fn db_path(&self) -> &Path {
        &self.db_path
    }

    /// Initialize the database schema.
    fn init_schema(&self) -> Result<()> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        self.runtime.block_on(async {
            db.query(SCHEMA_INIT).await.map_err(|e| {
                GotError::DatabaseError(format!("Failed to initialize schema: {e}"))
            })?;
            Ok::<_, GotError>(())
        })
    }

    /// Initialize HNSW index with the given dimension.
    fn init_hnsw_index(&self, dimension: usize) -> Result<()> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let query = format!(
            "DEFINE INDEX IF NOT EXISTS person_embedding_idx ON person FIELDS embedding \
             HNSW DIMENSION {dimension} DIST COSINE M 16 EFC 150;"
        );

        self.runtime.block_on(async {
            db.query(&query).await.map_err(|e| {
                GotError::DatabaseError(format!("Failed to create HNSW index: {e}"))
            })?;
            Ok::<_, GotError>(())
        })
    }

    /// Insert a person record into the database.
    fn insert_person_record(&self, record: &PersonRecord) -> Result<String> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let record_key = record.person_id.clone();
        let record_key_owned = record_key.clone();
        let record_clone = record.clone();

        self.runtime.block_on(async {
            db.query("CREATE type::thing('person', $record_id) CONTENT $content")
                .bind(("record_id", record_key_owned))
                .bind(("content", record_clone))
                .await
                .map_err(|e| GotError::DatabaseError(format!("Failed to insert person: {e}")))?;
            Ok::<_, GotError>(())
        })?;

        Ok(record_key)
    }
}

const SCHEMA_INIT: &str = r"
DEFINE NAMESPACE IF NOT EXISTS got;
USE NS got;
DEFINE DATABASE IF NOT EXISTS main;
USE DB main;

-- Person table (SCHEMAFULL for strict typing)
DEFINE TABLE IF NOT EXISTS person SCHEMAFULL;

-- Person fields
DEFINE FIELD IF NOT EXISTS person_id ON person TYPE string ASSERT $value != NONE;
DEFINE FIELD IF NOT EXISTS name ON person TYPE string;
DEFINE FIELD IF NOT EXISTS house ON person TYPE string;
DEFINE FIELD IF NOT EXISTS titles ON person TYPE array<string>;
DEFINE FIELD IF NOT EXISTS alias ON person TYPE option<string>;
DEFINE FIELD IF NOT EXISTS is_alive ON person TYPE bool;
DEFINE FIELD IF NOT EXISTS embedding ON person TYPE array<float>;

-- Unique index on person_id
DEFINE INDEX IF NOT EXISTS person_id_idx ON person FIELDS person_id UNIQUE;

-- House index for filtering
DEFINE INDEX IF NOT EXISTS person_house_idx ON person FIELDS house;

-- Relationship table (for graph edges)
DEFINE TABLE IF NOT EXISTS relates SCHEMAFULL TYPE RELATION IN person OUT person;
DEFINE FIELD IF NOT EXISTS label ON relates TYPE string;
DEFINE INDEX IF NOT EXISTS relates_label_idx ON relates FIELDS label;
";

impl GotBackend for SurrealDbBackend {
    fn new(db_path: &Path) -> Result<Self> {
        let runtime = Arc::new(
            Runtime::new()
                .map_err(|e| GotError::DatabaseError(format!("Failed to create runtime: {e}")))?,
        );

        std::fs::create_dir_all(db_path).map_err(|e| {
            GotError::DatabaseError(format!("Failed to create database directory: {e}"))
        })?;

        let db_path_owned = db_path.to_path_buf();
        let db = runtime.block_on(async {
            let path = db_path_owned.to_string_lossy().to_string();
            let db: Surreal<Db> = Surreal::new::<SurrealKv>(&path)
                .await
                .map_err(|e| GotError::DatabaseError(format!("Failed to open SurrealDB: {e}")))?;
            db.use_ns("got")
                .use_db("main")
                .await
                .map_err(|e| GotError::DatabaseError(format!("Failed to select namespace: {e}")))?;
            Ok::<_, GotError>(db)
        })?;

        let backend = Self {
            db: Some(db),
            db_path: db_path.to_path_buf(),
            runtime,
            id_to_record: HashMap::new(),
        };

        backend.init_schema()?;
        Ok(backend)
    }

    fn exists(db_path: &Path) -> bool {
        db_path.exists() && db_path.is_dir()
    }

    fn clear(&self) -> Result<()> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        self.runtime.block_on(async {
            db.query("DELETE person; DELETE relates;")
                .await
                .map_err(|e| GotError::DatabaseError(format!("Failed to clear database: {e}")))?;
            Ok::<_, GotError>(())
        })
    }

    fn ingest(&mut self, tree: &FamilyTree) -> Result<IngestStats> {
        let mut stats = IngestStats::default();

        // First pass: insert all people as nodes
        for person in &tree.people {
            let record = PersonRecord {
                person_id: person.id.clone(),
                name: person.name.clone(),
                house: person.house.to_string(),
                titles: person.titles.clone(),
                alias: person.alias.clone(),
                is_alive: person.is_alive,
                embedding: Vec::new(),
            };

            let record_key = self.insert_person_record(&record)?;
            self.id_to_record.insert(person.id.clone(), record_key);
            stats.nodes_inserted += 1;
        }

        // Second pass: create all relationship edges
        for rel in &tree.relationships {
            match rel {
                RelationshipDef::ParentOf { from, to } => {
                    let Some(from_key) = self.id_to_record.get(from) else {
                        continue;
                    };

                    for child_id in to {
                        let Some(to_key) = self.id_to_record.get(child_id) else {
                            continue;
                        };
                        self.create_edge(from_key, to_key, RelationType::ParentOf)?;
                        stats.edges_inserted += 1;
                    }
                }
                RelationshipDef::SpouseOf { between } => {
                    if between.len() >= 2 {
                        let Some(a) = self.id_to_record.get(&between[0]) else {
                            continue;
                        };
                        let Some(b) = self.id_to_record.get(&between[1]) else {
                            continue;
                        };
                        // Bidirectional
                        self.create_edge(a, b, RelationType::SpouseOf)?;
                        self.create_edge(b, a, RelationType::SpouseOf)?;
                        stats.edges_inserted += 2;
                    }
                }
                RelationshipDef::SiblingOf { between } => {
                    for i in 0..between.len() {
                        for j in (i + 1)..between.len() {
                            let Some(a) = self.id_to_record.get(&between[i]) else {
                                continue;
                            };
                            let Some(b) = self.id_to_record.get(&between[j]) else {
                                continue;
                            };
                            self.create_edge(a, b, RelationType::SiblingOf)?;
                            self.create_edge(b, a, RelationType::SiblingOf)?;
                            stats.edges_inserted += 2;
                        }
                    }
                }
            }
        }

        Ok(stats)
    }

    fn insert_person_basic(&self, person: &Person) -> Result<String> {
        let record = PersonRecord {
            person_id: person.id.clone(),
            name: person.name.clone(),
            house: person.house.to_string(),
            titles: person.titles.clone(),
            alias: person.alias.clone(),
            is_alive: person.is_alive,
            embedding: Vec::new(),
        };
        self.insert_person_record(&record)
    }

    fn insert_person_with_embedding(
        &self,
        person: &Person,
        embedding: &[f32],
    ) -> Result<(String, String)> {
        // Initialize HNSW index with embedding dimension on first use
        if !embedding.is_empty() {
            self.init_hnsw_index(embedding.len())?;
        }

        let record = PersonRecord {
            person_id: person.id.clone(),
            name: person.name.clone(),
            house: person.house.to_string(),
            titles: person.titles.clone(),
            alias: person.alias.clone(),
            is_alive: person.is_alive,
            embedding: embedding.to_vec(),
        };

        let record_key = self.insert_person_record(&record)?;
        // For SurrealDB, the vector is stored in the same record, so vector_id = record_key
        Ok((record_key.clone(), record_key))
    }

    fn create_edge(
        &self,
        from_node_id: &str,
        to_node_id: &str,
        relation_type: RelationType,
    ) -> Result<()> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let label = relation_type.as_edge_label().to_string();
        let from_id = from_node_id.to_string();
        let to_id = to_node_id.to_string();

        self.runtime.block_on(async {
            db.query("RELATE $from->relates->$to SET label = $label")
                .bind((
                    "from",
                    surrealdb::RecordId::from(("person", from_id.as_str())),
                ))
                .bind(("to", surrealdb::RecordId::from(("person", to_id.as_str()))))
                .bind(("label", label))
                .await
                .map_err(|e| GotError::DatabaseError(format!("Failed to create edge: {e}")))?;
            Ok::<_, GotError>(())
        })
    }

    fn search_semantic(&self, embedding: &[f32], limit: usize) -> Result<Vec<SearchResult>> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let embedding_vec = embedding.to_vec();
        let ef_search = 64;

        let query = format!(
            "SELECT person_id, name, house, \
                    string::join(',', titles) AS titles, \
                    alias ?? '' AS alias, \
                    <string>is_alive AS is_alive, \
                    vector::distance::knn() AS distance \
             FROM person \
             WHERE embedding <|{limit},{ef_search}|> $query_embedding \
             ORDER BY distance"
        );

        let results: Vec<VectorSearchResult> = self.runtime.block_on(async {
            db.query(&query)
                .bind(("query_embedding", embedding_vec))
                .await
                .map_err(|e| GotError::VectorSearchError(format!("Search failed: {e}")))?
                .take(0)
                .map_err(|e| GotError::VectorSearchError(format!("Failed to parse results: {e}")))
        })?;

        let mut search_results = Vec::new();
        for r in results {
            // Convert distance to score
            #[allow(clippy::cast_possible_truncation)]
            let score = (1.0 / (1.0 + r.distance)) as f32;

            let house: House = r.house.parse().map_err(|e| {
                GotError::DatabaseError(format!("Invalid house '{}': {e}", r.house))
            })?;

            let titles: Vec<String> = if r.titles.is_empty() {
                Vec::new()
            } else {
                r.titles.split(',').map(|s| s.trim().to_string()).collect()
            };

            let alias = if r.alias.is_empty() {
                None
            } else {
                Some(r.alias)
            };

            let is_alive = r.is_alive.parse().unwrap_or(true);

            search_results.push(SearchResult {
                person: Person {
                    id: r.person_id,
                    name: r.name,
                    house,
                    titles,
                    alias,
                    is_alive,
                },
                score,
                snippet: None,
            });
        }

        Ok(search_results)
    }

    fn lookup_by_id(&self, person_id: &str) -> Result<Option<String>> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let person_id_owned = person_id.to_string();

        #[derive(Deserialize)]
        struct IdResult {
            person_id: String,
        }

        let results: Vec<IdResult> = self.runtime.block_on(async {
            db.query("SELECT person_id FROM person WHERE person_id = $pid LIMIT 1")
                .bind(("pid", person_id_owned))
                .await
                .map_err(|e| GotError::DatabaseError(format!("Lookup failed: {e}")))?
                .take(0)
                .map_err(|e| GotError::DatabaseError(format!("Failed to parse lookup: {e}")))
        })?;

        Ok(results.first().map(|r| r.person_id.clone()))
    }

    fn get_person(&self, node_id: &str) -> Result<Person> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let node_id_owned = node_id.to_string();

        let results: Vec<PersonRecord> = self.runtime.block_on(async {
            db.query("SELECT * FROM person WHERE person_id = $pid LIMIT 1")
                .bind(("pid", node_id_owned))
                .await
                .map_err(|e| GotError::DatabaseError(format!("Get person failed: {e}")))?
                .take(0)
                .map_err(|e| GotError::DatabaseError(format!("Failed to parse person: {e}")))
        })?;

        let record = results
            .into_iter()
            .next()
            .ok_or_else(|| GotError::PersonNotFound(node_id.to_string()))?;

        let house: House = record.house.parse().map_err(|e| {
            GotError::DatabaseError(format!("Invalid house '{}': {e}", record.house))
        })?;

        Ok(Person {
            id: record.person_id,
            name: record.name,
            house,
            titles: record.titles,
            alias: record.alias,
            is_alive: record.is_alive,
        })
    }

    fn get_incoming_neighbors(
        &self,
        node_id: &str,
        relation_type: RelationType,
    ) -> Result<Vec<String>> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let label = relation_type.as_edge_label().to_string();
        let node_id_owned = node_id.to_string();

        let results: Vec<NeighborResult> = self.runtime.block_on(async {
            db.query(
                "SELECT in.person_id AS person_id FROM relates \
                 WHERE out.person_id = $to_id AND label = $label",
            )
            .bind(("to_id", node_id_owned))
            .bind(("label", label))
            .await
            .map_err(|e| GotError::DatabaseError(format!("Get neighbors failed: {e}")))?
            .take(0)
            .map_err(|e| GotError::DatabaseError(format!("Failed to parse neighbors: {e}")))
        })?;

        let mut neighbors: Vec<String> = results.into_iter().map(|r| r.person_id).collect();
        neighbors.sort();
        neighbors.dedup();
        Ok(neighbors)
    }

    fn get_outgoing_neighbors(
        &self,
        node_id: &str,
        relation_type: RelationType,
    ) -> Result<Vec<String>> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let label = relation_type.as_edge_label().to_string();
        let node_id_owned = node_id.to_string();

        let results: Vec<NeighborResult> = self.runtime.block_on(async {
            db.query(
                "SELECT out.person_id AS person_id FROM relates \
                 WHERE in.person_id = $from_id AND label = $label",
            )
            .bind(("from_id", node_id_owned))
            .bind(("label", label))
            .await
            .map_err(|e| GotError::DatabaseError(format!("Get neighbors failed: {e}")))?
            .take(0)
            .map_err(|e| GotError::DatabaseError(format!("Failed to parse neighbors: {e}")))
        })?;

        let mut neighbors: Vec<String> = results.into_iter().map(|r| r.person_id).collect();
        neighbors.sort();
        neighbors.dedup();
        Ok(neighbors)
    }

    fn get_stats(&self) -> Result<GraphStats> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let (node_count, edge_count, house_counts): (usize, usize, HashMap<String, usize>) =
            self.runtime.block_on(async {
                // Count nodes
                let node_results: Vec<CountResult> = db
                    .query("SELECT count() AS count FROM person GROUP ALL")
                    .await
                    .map_err(|e| GotError::DatabaseError(format!("Count failed: {e}")))?
                    .take(0)
                    .map_err(|e| GotError::DatabaseError(format!("Failed to parse count: {e}")))?;
                let node_count = node_results.first().map(|r| r.count).unwrap_or(0);

                // Count edges
                let edge_results: Vec<CountResult> = db
                    .query("SELECT count() AS count FROM relates GROUP ALL")
                    .await
                    .map_err(|e| GotError::DatabaseError(format!("Edge count failed: {e}")))?
                    .take(0)
                    .map_err(|e| {
                        GotError::DatabaseError(format!("Failed to parse edge count: {e}"))
                    })?;
                let edge_count = edge_results.first().map(|r| r.count).unwrap_or(0);

                // Count by house
                let house_results: Vec<HouseCountResult> = db
                    .query("SELECT house, count() AS count FROM person GROUP BY house")
                    .await
                    .map_err(|e| GotError::DatabaseError(format!("House count failed: {e}")))?
                    .take(0)
                    .map_err(|e| {
                        GotError::DatabaseError(format!("Failed to parse house count: {e}"))
                    })?;

                let house_counts: HashMap<String, usize> = house_results
                    .into_iter()
                    .map(|r| (r.house, r.count))
                    .collect();

                Ok::<_, GotError>((node_count, edge_count, house_counts))
            })?;

        Ok(GraphStats {
            node_count,
            edge_count,
            house_counts,
        })
    }

    fn get_house_members(&self, house: House) -> Result<Vec<Person>> {
        let db = self
            .db
            .as_ref()
            .ok_or_else(|| GotError::DatabaseError("Database not initialized".to_string()))?;

        let house_str = house.to_string();

        let results: Vec<PersonRecord> = self.runtime.block_on(async {
            db.query("SELECT * FROM person WHERE house = $house")
                .bind(("house", house_str))
                .await
                .map_err(|e| GotError::DatabaseError(format!("Query failed: {e}")))?
                .take(0)
                .map_err(|e| GotError::DatabaseError(format!("Failed to parse results: {e}")))
        })?;

        let mut members = Vec::new();
        for record in results {
            let house: House = record.house.parse().map_err(|e| {
                GotError::DatabaseError(format!("Invalid house '{}': {e}", record.house))
            })?;

            members.push(Person {
                id: record.person_id,
                name: record.name,
                house,
                titles: record.titles,
                alias: record.alias,
                is_alive: record.is_alive,
            });
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

    fn open_storage() -> Result<(TempDir, SurrealDbBackend)> {
        let temp = TempDir::new()?;
        let storage = SurrealDbBackend::new(temp.path())?;
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
