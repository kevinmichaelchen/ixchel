//! CLI for the Game of Thrones family tree demo.

use clap::{Parser, Subcommand, ValueEnum};
use demo_got::{
    BioLoader, FamilyTree, GotBackend, HelixDbBackend, House, RelationType, RelationshipDef,
    SurrealDbBackend, find_ancestors, find_descendants, get_person_with_family,
};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum Backend {
    /// HelixDB storage backend
    Helixdb,
    /// SurrealDB storage backend
    #[default]
    Surrealdb,
}

#[derive(Parser)]
#[command(name = "demo-got")]
#[command(about = "Game of Thrones family tree graph demo with pluggable storage backends")]
#[command(version)]
struct Cli {
    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,

    /// Path to the database directory
    #[arg(long, global = true)]
    db_path: Option<PathBuf>,

    /// Storage backend to use
    #[arg(long, global = true, value_enum, default_value = "surrealdb")]
    backend: Backend,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ingest family tree data from a YAML file
    Ingest {
        /// Path to the YAML file (default: data/westeros.yaml relative to crate)
        #[arg(short, long)]
        file: Option<PathBuf>,

        /// Clear existing data before ingesting
        #[arg(long)]
        clear: bool,

        /// Skip generating embeddings (faster for development iteration)
        #[arg(long)]
        skip_embeddings: bool,
    },

    /// Semantic search across character bios
    Search {
        /// Search query (natural language)
        query: String,

        /// Maximum number of results to return
        #[arg(short, long, default_value = "5")]
        limit: usize,
    },

    /// Query the family tree graph
    Query {
        #[command(subcommand)]
        query_type: QueryType,
    },

    /// Show database statistics
    Stats,
}

#[derive(Subcommand)]
enum QueryType {
    /// Find all ancestors of a person
    Ancestors {
        /// Person ID (e.g., "jon-snow")
        person_id: String,

        /// Maximum depth to traverse
        #[arg(short, long, default_value = "10")]
        depth: usize,
    },

    /// Find all descendants of a person
    Descendants {
        /// Person ID (e.g., "ned-stark")
        person_id: String,

        /// Maximum depth to traverse
        #[arg(short, long, default_value = "10")]
        depth: usize,
    },

    /// Find all members of a house
    House {
        /// House name (stark, targaryen, baratheon, tully, lannister)
        house: String,
    },

    /// Show person details and immediate family
    Person {
        /// Person ID (e.g., "jon-snow")
        person_id: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(1)
        }
    }
}

fn run(cli: Cli) -> anyhow::Result<()> {
    // Default database path includes backend name to avoid conflicts
    let backend_suffix = match cli.backend {
        Backend::Helixdb => "helixdb",
        Backend::Surrealdb => "surrealdb",
    };
    let db_path = cli.db_path.unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(".data-{backend_suffix}"))
    });

    match cli.backend {
        Backend::Helixdb => run_with_backend::<HelixDbBackend>(cli.json, &db_path, cli.command),
        Backend::Surrealdb => run_with_backend::<SurrealDbBackend>(cli.json, &db_path, cli.command),
    }
}

fn run_with_backend<B: GotBackend>(
    json: bool,
    db_path: &std::path::Path,
    command: Commands,
) -> anyhow::Result<()> {
    match command {
        Commands::Ingest {
            file,
            clear,
            skip_embeddings,
        } => {
            let yaml_path = file.unwrap_or_else(|| {
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/westeros.yaml")
            });

            println!("Loading family tree from: {}", yaml_path.display());
            let tree = FamilyTree::load(&yaml_path)?;
            println!(
                "Loaded {} people and {} relationship definitions",
                tree.people.len(),
                tree.relationships.len()
            );

            // Load biographies
            let data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data");
            let bios = BioLoader::load_all(&data_dir)?;
            println!("Loaded {} character biographies", bios.len());

            let mut storage = B::new(db_path)?;

            if clear {
                println!("Clearing existing data...");
                storage.clear()?;
            }

            println!("Ingesting into database at {}...", db_path.display());

            if skip_embeddings {
                let stats = storage.ingest(&tree)?;
                println!(
                    "Ingested {} nodes and {} edges (no embeddings)",
                    stats.nodes_inserted, stats.edges_inserted
                );
            } else {
                println!("Initializing embedding model...");
                let embedder = ix_embeddings::Embedder::new()
                    .map_err(|e| anyhow::anyhow!("Failed to create embedder: {e}"))?;

                // Build composite texts for people with bios
                let mut texts_with_ids: Vec<(String, String)> = Vec::new();
                for person in &tree.people {
                    if let Some(bio) = bios.get(&person.id) {
                        let composite = bio.composite_text(person);
                        texts_with_ids.push((person.id.clone(), composite));
                    }
                }

                if texts_with_ids.is_empty() {
                    println!("Warning: No biographies found, falling back to basic ingest");
                    let stats = storage.ingest(&tree)?;
                    println!(
                        "Ingested {} nodes and {} edges (no embeddings)",
                        stats.nodes_inserted, stats.edges_inserted
                    );
                } else {
                    println!(
                        "Generating embeddings for {} characters...",
                        texts_with_ids.len()
                    );
                    let texts: Vec<&str> = texts_with_ids.iter().map(|(_, t)| t.as_str()).collect();
                    let embeddings = embedder
                        .embed_batch(&texts)
                        .map_err(|e| anyhow::anyhow!("Failed to generate embeddings: {e}"))?;

                    // Create a map of person_id -> embedding
                    let embedding_map: std::collections::HashMap<String, Vec<f32>> = texts_with_ids
                        .iter()
                        .zip(embeddings)
                        .map(|((id, _), emb)| (id.clone(), emb))
                        .collect();

                    // Ingest with embeddings
                    let stats = ingest_with_embeddings(&mut storage, &tree, &embedding_map)?;
                    println!(
                        "Ingested {} nodes and {} edges ({} with embeddings)",
                        stats.nodes_inserted, stats.edges_inserted, stats.embeddings_inserted
                    );
                }
            }

            // Verify persistence
            let db_stats = storage.get_stats()?;
            println!("\nDatabase verification:");
            println!("  Nodes in DB: {}", db_stats.node_count);
            println!("  Edges in DB: {}", db_stats.edge_count);
            println!("  Houses: {:?}", db_stats.house_counts);
        }

        Commands::Search { query, limit } => {
            if !B::exists(db_path) {
                anyhow::bail!(
                    "Database not found at {}. Run 'demo-got ingest' first.",
                    db_path.display()
                );
            }

            let storage = B::new(db_path)?;

            // Generate query embedding
            let embedder = ix_embeddings::Embedder::new()
                .map_err(|e| anyhow::anyhow!("Failed to create embedder: {e}"))?;
            let query_embedding = embedder
                .embed(&query)
                .map_err(|e| anyhow::anyhow!("Failed to embed query: {e}"))?;

            // Perform semantic search
            let results = storage.search_semantic(&query_embedding, limit)?;

            if json {
                let output: Vec<_> = results
                    .iter()
                    .map(|r| {
                        serde_json::json!({
                            "id": r.person.id,
                            "name": r.person.name,
                            "house": r.person.house.to_string(),
                            "alias": r.person.alias,
                            "score": r.score,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("Search results for: \"{}\"", query);
                println!();
                if results.is_empty() {
                    println!("  (no results found)");
                } else {
                    for (i, result) in results.iter().enumerate() {
                        let alias = result
                            .person
                            .alias
                            .as_ref()
                            .map(|a| format!(" \"{}\"", a))
                            .unwrap_or_default();
                        println!(
                            "{}. {}{} (House {}) - score: {:.3}",
                            i + 1,
                            result.person.name,
                            alias,
                            result.person.house,
                            result.score
                        );
                    }
                }
            }
        }

        Commands::Query { query_type } => {
            if !B::exists(db_path) {
                anyhow::bail!(
                    "Database not found at {}. Run 'demo-got ingest' first.",
                    db_path.display()
                );
            }

            let storage = B::new(db_path)?;

            match query_type {
                QueryType::Ancestors { person_id, depth } => {
                    let ancestors = find_ancestors(&storage, &person_id, depth)?;

                    if json {
                        let output: Vec<_> = ancestors
                            .iter()
                            .map(|a| {
                                serde_json::json!({
                                    "id": a.person.id,
                                    "name": a.person.name,
                                    "house": a.person.house.to_string(),
                                    "depth": a.depth,
                                })
                            })
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    } else {
                        println!("Ancestors of {}:", person_id);
                        if ancestors.is_empty() {
                            println!("  (none found)");
                        } else {
                            for ancestor in &ancestors {
                                let indent = "  ".repeat(ancestor.depth as usize);
                                let alias = ancestor
                                    .person
                                    .alias
                                    .as_ref()
                                    .map(|a| format!(" \"{}\"", a))
                                    .unwrap_or_default();
                                println!(
                                    "{}{}{} (House {})",
                                    indent, ancestor.person.name, alias, ancestor.person.house
                                );
                            }
                        }
                    }
                }

                QueryType::Descendants { person_id, depth } => {
                    let descendants = find_descendants(&storage, &person_id, depth)?;

                    if json {
                        let output: Vec<_> = descendants
                            .iter()
                            .map(|d| {
                                serde_json::json!({
                                    "id": d.person.id,
                                    "name": d.person.name,
                                    "house": d.person.house.to_string(),
                                    "depth": d.depth,
                                })
                            })
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    } else {
                        println!("Descendants of {}:", person_id);
                        if descendants.is_empty() {
                            println!("  (none found)");
                        } else {
                            for descendant in &descendants {
                                let indent = "  ".repeat(descendant.depth as usize);
                                println!(
                                    "{}{} (House {})",
                                    indent, descendant.person.name, descendant.person.house
                                );
                            }
                        }
                    }
                }

                QueryType::House { house } => {
                    let house: House = house
                        .parse()
                        .map_err(|e| anyhow::anyhow!("Invalid house: {}", e))?;

                    let members = storage.get_house_members(house)?;

                    if json {
                        let output: Vec<_> = members
                            .iter()
                            .map(|p| {
                                serde_json::json!({
                                    "id": p.id,
                                    "name": p.name,
                                    "alias": p.alias,
                                    "titles": p.titles,
                                    "is_alive": p.is_alive,
                                })
                            })
                            .collect();
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    } else {
                        println!("Members of House {}:", house);
                        for person in &members {
                            let alias = person
                                .alias
                                .as_ref()
                                .map(|a| format!(" \"{}\"", a))
                                .unwrap_or_default();
                            let status = if person.is_alive { "" } else { " [deceased]" };
                            println!("  {}{}{}", person.name, alias, status);
                        }
                        println!("\nTotal: {} members", members.len());
                    }
                }

                QueryType::Person { person_id } => {
                    let family = get_person_with_family(&storage, &person_id)?;

                    if json {
                        let output = serde_json::json!({
                            "person": {
                                "id": family.person.id,
                                "name": family.person.name,
                                "house": family.person.house.to_string(),
                                "alias": family.person.alias,
                                "titles": family.person.titles,
                                "is_alive": family.person.is_alive,
                            },
                            "parents": family.parents.iter().map(|p| {
                                serde_json::json!({ "id": p.id, "name": p.name })
                            }).collect::<Vec<_>>(),
                            "spouses": family.spouses.iter().map(|p| {
                                serde_json::json!({ "id": p.id, "name": p.name })
                            }).collect::<Vec<_>>(),
                            "children": family.children.iter().map(|p| {
                                serde_json::json!({ "id": p.id, "name": p.name })
                            }).collect::<Vec<_>>(),
                            "siblings": family.siblings.iter().map(|p| {
                                serde_json::json!({ "id": p.id, "name": p.name })
                            }).collect::<Vec<_>>(),
                        });
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    } else {
                        println!("{}", family.person.name);
                        if let Some(alias) = &family.person.alias {
                            println!("  Alias: \"{}\"", alias);
                        }
                        println!("  House: {}", family.person.house);
                        if !family.person.titles.is_empty() {
                            println!("  Titles: {}", family.person.titles.join(", "));
                        }
                        println!(
                            "  Status: {}",
                            if family.person.is_alive {
                                "Alive"
                            } else {
                                "Deceased"
                            }
                        );

                        if !family.parents.is_empty() {
                            println!("\nParents:");
                            for parent in &family.parents {
                                println!("  - {} ({})", parent.name, parent.house);
                            }
                        }

                        if !family.spouses.is_empty() {
                            println!("\nSpouse(s):");
                            for spouse in &family.spouses {
                                println!("  - {} ({})", spouse.name, spouse.house);
                            }
                        }

                        if !family.children.is_empty() {
                            println!("\nChildren:");
                            for child in &family.children {
                                println!("  - {} ({})", child.name, child.house);
                            }
                        }

                        if !family.siblings.is_empty() {
                            println!("\nSiblings:");
                            for sibling in &family.siblings {
                                println!("  - {} ({})", sibling.name, sibling.house);
                            }
                        }
                    }
                }
            }
        }

        Commands::Stats => {
            if !B::exists(db_path) {
                anyhow::bail!(
                    "Database not found at {}. Run 'demo-got ingest' first.",
                    db_path.display()
                );
            }

            let storage = B::new(db_path)?;
            let stats = storage.get_stats()?;

            if json {
                println!("{}", serde_json::to_string_pretty(&stats)?);
            } else {
                println!("Database Statistics");
                println!("==================");
                println!("Path: {}", db_path.display());
                println!("Nodes: {}", stats.node_count);
                println!("Edges: {}", stats.edge_count);
                println!("\nHouse breakdown:");
                for (house, count) in &stats.house_counts {
                    println!("  {}: {} members", house, count);
                }
            }
        }
    }

    Ok(())
}

/// Statistics from an ingest operation with embeddings.
struct IngestWithEmbeddingsStats {
    nodes_inserted: usize,
    edges_inserted: usize,
    embeddings_inserted: usize,
}

/// Ingest family tree data with embeddings for semantic search.
fn ingest_with_embeddings<B: GotBackend>(
    storage: &mut B,
    tree: &FamilyTree,
    embeddings: &std::collections::HashMap<String, Vec<f32>>,
) -> anyhow::Result<IngestWithEmbeddingsStats> {
    let mut stats = IngestWithEmbeddingsStats {
        nodes_inserted: 0,
        edges_inserted: 0,
        embeddings_inserted: 0,
    };

    // Track person ID -> node ID mapping for relationship creation
    let mut id_to_node: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    // First pass: insert all people as nodes
    for person in &tree.people {
        let (node_id, used_embedding) = if let Some(embedding) = embeddings.get(&person.id) {
            // Insert with embedding
            let (node_id, _vector_id) = storage.insert_person_with_embedding(person, embedding)?;
            (node_id, true)
        } else {
            // Fall back to basic insert so the graph stays complete.
            let node_id = storage.insert_person_basic(person)?;
            (node_id, false)
        };
        if used_embedding {
            stats.embeddings_inserted += 1;
        }
        id_to_node.insert(person.id.clone(), node_id);
        stats.nodes_inserted += 1;
    }

    // Second pass: create all relationship edges
    for rel in &tree.relationships {
        match rel {
            RelationshipDef::ParentOf { from, to } => {
                let Some(from_node) = id_to_node.get(from) else {
                    continue;
                };

                for child_id in to {
                    let Some(to_node) = id_to_node.get(child_id) else {
                        continue;
                    };
                    storage.create_edge(from_node, to_node, RelationType::ParentOf)?;
                    stats.edges_inserted += 1;
                }
            }
            RelationshipDef::SpouseOf { between } => {
                if between.len() >= 2 {
                    let Some(a) = id_to_node.get(&between[0]) else {
                        continue;
                    };
                    let Some(b) = id_to_node.get(&between[1]) else {
                        continue;
                    };
                    // Bidirectional: create edges in both directions
                    storage.create_edge(a, b, RelationType::SpouseOf)?;
                    storage.create_edge(b, a, RelationType::SpouseOf)?;
                    stats.edges_inserted += 2;
                }
            }
            RelationshipDef::SiblingOf { between } => {
                // Create edges between all pairs (bidirectional)
                for i in 0..between.len() {
                    for j in (i + 1)..between.len() {
                        let Some(a) = id_to_node.get(&between[i]) else {
                            continue;
                        };
                        let Some(b) = id_to_node.get(&between[j]) else {
                            continue;
                        };
                        storage.create_edge(a, b, RelationType::SiblingOf)?;
                        storage.create_edge(b, a, RelationType::SiblingOf)?;
                        stats.edges_inserted += 2;
                    }
                }
            }
        }
    }

    Ok(stats)
}
