# Graph Schema Specification

This document defines the complete graph schema for the Helix knowledge system, including node types, edge types, properties, and query patterns.

## Overview

The Helix graph is stored in HelixDB, which provides:

- **Graph storage**: Typed nodes and edges with properties
- **Vector storage**: HNSW index for semantic similarity
- **Secondary indices**: Fast lookups by property values
- **ACID transactions**: Consistent reads and writes

## Node Labels

Each entity type maps to a node label in the graph.

```
┌──────────────────────────────────────────────────────────────────┐
│                         Node Labels                               │
├────────────┬─────────────────────────────────────────────────────┤
│ Label      │ Description                                         │
├────────────┼─────────────────────────────────────────────────────┤
│ DECISION   │ Architecture Decision Records                       │
│ ISSUE      │ Work items: bugs, features, tasks                   │
│ IDEA       │ Proposals and brainstorms                           │
│ REPORT     │ Analysis documents: postmortems, RFCs               │
│ SOURCE     │ External references: papers, articles               │
│ CITATION   │ Specific quotes from sources                        │
└────────────┴─────────────────────────────────────────────────────┘
```

## Node Properties

### Common Properties (All Nodes)

| Property          | Type     | Required | Indexed      | Description           |
| ----------------- | -------- | -------- | ------------ | --------------------- |
| `id`              | String   | Yes      | Yes (unique) | Entity ID with prefix |
| `uuid`            | String   | Yes      | Yes          | Content-based UUID    |
| `title`           | String   | Yes      | No           | Human-readable title  |
| `body`            | String   | No       | No           | Markdown body content |
| `created_at`      | DateTime | Yes      | Yes          | Creation timestamp    |
| `updated_at`      | DateTime | Yes      | Yes          | Last modification     |
| `created_by`      | String   | No       | Yes          | Creator name/handle   |
| `created_by_type` | String   | No       | No           | "human" or "agent"    |
| `agent_id`        | String   | No       | No           | Agent identifier      |
| `session_id`      | String   | No       | No           | Agent session         |
| `tags`            | String[] | No       | Yes          | Labels/categories     |
| `external_ref`    | String   | No       | No           | External system link  |
| `content_hash`    | String   | Yes      | No           | SHA256 of content     |
| `file_path`       | String   | Yes      | No           | Relative file path    |
| `vector_id`       | String   | No       | Yes          | Link to HNSW vector   |

### DECISION Properties

| Property   | Type     | Required | Indexed | Description                                      |
| ---------- | -------- | -------- | ------- | ------------------------------------------------ |
| `status`   | String   | Yes      | Yes     | proposed/accepted/rejected/superseded/deprecated |
| `date`     | Date     | Yes      | No      | Decision date                                    |
| `deciders` | String[] | No       | No      | Decision makers                                  |

### ISSUE Properties

| Property            | Type     | Required | Indexed | Description                     |
| ------------------- | -------- | -------- | ------- | ------------------------------- |
| `status`            | String   | Yes      | Yes     | open/in_progress/blocked/closed |
| `issue_type`        | String   | No       | Yes     | bug/feature/task/epic/chore     |
| `priority`          | Int      | No       | Yes     | 0-4 (0=critical)                |
| `assignee`          | String   | No       | Yes     | Assigned person                 |
| `parent_id`         | String   | No       | Yes     | Parent epic/task                |
| `closed_at`         | DateTime | No       | No      | When closed                     |
| `closed_reason`     | String   | No       | No      | Why closed                      |
| `estimated_minutes` | Int      | No       | No      | Time estimate                   |

### IDEA Properties

| Property   | Type   | Required | Indexed | Description                            |
| ---------- | ------ | -------- | ------- | -------------------------------------- |
| `status`   | String | Yes      | Yes     | draft/proposed/parked/rejected/evolved |
| `champion` | String | No       | No      | Advocate                               |
| `effort`   | String | No       | No      | low/medium/high/unknown                |
| `impact`   | String | No       | No      | low/medium/high/unknown                |

### REPORT Properties

| Property        | Type     | Required | Indexed | Description                                    |
| --------------- | -------- | -------- | ------- | ---------------------------------------------- |
| `status`        | String   | Yes      | Yes     | draft/published/archived                       |
| `report_type`   | String   | Yes      | Yes     | postmortem/rfc/retrospective/analysis/research |
| `period_start`  | Date     | No       | No      | Period start (retrospectives)                  |
| `period_end`    | Date     | No       | No      | Period end                                     |
| `incident_date` | DateTime | No       | No      | Incident time (postmortems)                    |

### SOURCE Properties

| Property         | Type     | Required | Indexed | Description                                            |
| ---------------- | -------- | -------- | ------- | ------------------------------------------------------ |
| `source_type`    | String   | Yes      | Yes     | paper/article/documentation/book/talk/video/repository |
| `url`            | String   | No       | No      | Original location                                      |
| `authors`        | String[] | No       | No      | Author names                                           |
| `published_date` | Date     | No       | No      | Publication date                                       |
| `publisher`      | String   | No       | No      | Journal, blog, etc.                                    |
| `doi`            | String   | No       | No      | DOI identifier                                         |
| `isbn`           | String   | No       | No      | ISBN for books                                         |
| `archived_at`    | String   | No       | No      | Archive.org URL                                        |
| `local_path`     | String   | No       | No      | Local file path                                        |

### CITATION Properties

| Property        | Type   | Required | Indexed | Description            |
| --------------- | ------ | -------- | ------- | ---------------------- |
| `quote`         | String | Yes      | No      | The quoted text        |
| `page`          | String | No       | No      | Page/section reference |
| `timestamp`     | String | No       | No      | Video timestamp        |
| `is_paraphrase` | Bool   | No       | No      | True if paraphrased    |

## Edge Types

### Edge Schema

```
┌──────────────────────────────────────────────────────────────────────────┐
│                            Edge Types                                     │
├───────────────┬──────────────────────────────────────────────────────────┤
│ Type          │ Description                                              │
├───────────────┼──────────────────────────────────────────────────────────┤
│ RELATES_TO    │ General association (bidirectional semantics)            │
│ BLOCKS        │ A must complete before B can proceed                     │
│ DEPENDS_ON    │ A requires B to exist/be complete                        │
│ SUPERSEDES    │ A replaces B (B is now obsolete)                         │
│ AMENDS        │ A modifies B (B still valid, with changes)               │
│ EVOLVES_INTO  │ A became B (A → B transformation)                        │
│ SPAWNS        │ A created B (parent-child creation)                      │
│ CITES         │ A references B as supporting material                    │
│ QUOTES        │ A directly excerpts text from B                          │
│ SUPPORTS      │ A provides evidence for B                                │
│ CONTRADICTS   │ A conflicts with B                                       │
│ SUMMARIZES    │ A condenses/aggregates B                                 │
│ ADDRESSES     │ A responds to/resolves B                                 │
│ IMPLEMENTS    │ A is the implementation of B                             │
│ FROM_SOURCE   │ Citation → Source relationship                           │
│ USED_IN       │ Citation used in Report/Decision                         │
└───────────────┴──────────────────────────────────────────────────────────┘
```

### Edge Validity Matrix

Which edges are valid between which node types:

```
                      TO
           ┌─────┬─────┬─────┬─────┬─────┬─────┐
           │ DEC │ ISS │IDEA │ RPT │ SRC │CITE │
    ┌──────┼─────┼─────┼─────┼─────┼─────┼─────┤
    │ DEC  │ S,A,│  Sp │  -  │  -  │  -  │  -  │
    │      │ D,R │     │     │     │     │     │
    ├──────┼─────┼─────┼─────┼─────┼─────┼─────┤
    │ ISS  │  Im │ B,D,│  -  │  -  │  -  │  -  │
    │      │     │  R  │     │     │     │     │
FROM├──────┼─────┼─────┼─────┼─────┼─────┼─────┤
    │ IDEA │  Ev │ Ev  │  In,│  -  │  -  │  -  │
    │      │     │     │  R  │     │     │     │
    ├──────┼─────┼─────┼─────┼─────┼─────┼─────┤
    │ RPT  │ Rec │ Sum │ Rec │  R  │  C  │  -  │
    │      │     │     │     │     │     │     │
    ├──────┼─────┼─────┼─────┼─────┼─────┼─────┤
    │ SRC  │  -  │  -  │  -  │  -  │  R  │  -  │
    │      │     │     │     │     │     │     │
    ├──────┼─────┼─────┼─────┼─────┼─────┼─────┤
    │ CITE │ Sup,│  -  │  -  │  UI │  FS │  -  │
    │      │ Con │     │     │     │     │     │
    └──────┴─────┴─────┴─────┴─────┴─────┴─────┘

Legend:
  S = SUPERSEDES       A = AMENDS         D = DEPENDS_ON
  R = RELATES_TO       Sp = SPAWNS        Im = IMPLEMENTS
  B = BLOCKS           Ev = EVOLVES_INTO  In = INSPIRED_BY
  C = CITES            Rec = RECOMMENDS   Sum = SUMMARIZES
  Sup = SUPPORTS       Con = CONTRADICTS  FS = FROM_SOURCE
  UI = USED_IN
```

### Edge Properties

All edges have these optional properties:

| Property     | Type     | Description                   |
| ------------ | -------- | ----------------------------- |
| `created_at` | DateTime | When relationship was created |
| `created_by` | String   | Who created the relationship  |
| `note`       | String   | Optional annotation           |

## Secondary Indices

For fast property-based lookups:

```rust
SecondaryIndex::Unique("id")        // O(1) lookup by entity ID
SecondaryIndex::Index("uuid")       // O(1) lookup by UUID
SecondaryIndex::Index("vector_id")  // Link vectors to nodes
SecondaryIndex::Index("status")     // Filter by status
SecondaryIndex::Index("created_by") // Filter by creator
SecondaryIndex::Index("tags")       // Filter by tag (multi-value)
SecondaryIndex::Index("assignee")   // Issues by assignee
SecondaryIndex::Index("priority")   // Issues by priority
SecondaryIndex::Index("parent_id")  // Child issues lookup
```

## Vector Index Configuration

```rust
VectorConfig {
    dimensions: 384,           // BGE-small-en-v1.5
    distance_metric: Cosine,   // Normalized embeddings
    hnsw: HnswConfig {
        m: 16,                 // Max connections per node
        ef_construction: 200,  // Build-time beam width
        ef_search: 64,         // Query-time beam width
    },
}
```

### Embedding Strategy

Each entity is embedded using:

```
{title}

{body}

Tags: {tags.join(", ")}
```

This ensures:

1. Title matches strongly (appears first)
2. Body content is fully searchable
3. Tags influence similarity

## Query Patterns

### 1. Find Entity by ID

```
MATCH (n {id: $id})
RETURN n
```

**HelixDB Implementation:**

```rust
storage.secondary_index_lookup("id", id)
```

### 2. List Entities by Type with Filters

```
MATCH (n:ISSUE)
WHERE n.status IN ['open', 'in_progress']
  AND n.priority <= 2
  AND 'database' IN n.tags
RETURN n
ORDER BY n.priority ASC, n.updated_at DESC
LIMIT 50
```

**HelixDB Implementation:**

```rust
storage.nodes()
    .with_label(Label::Issue)
    .filter(|n| filters.matches(n))
    .sort_by(|n| (n.priority, Reverse(n.updated_at)))
    .take(50)
```

### 3. Semantic Search

```
// Pseudo-query (vector search is separate)
CALL vector.search($query_embedding, 50)
YIELD node, score
WHERE node.status IN $allowed_statuses
  AND (node:DECISION OR node:ISSUE OR node:SOURCE)
RETURN node, score
ORDER BY score DESC
LIMIT 10
```

**HelixDB Implementation:**

```rust
// 1. Vector search
let candidates = storage.vectors.search(query_embedding, 50);

// 2. Resolve to nodes
let nodes = candidates
    .iter()
    .filter_map(|v| storage.secondary_index_lookup("vector_id", v.id))
    .collect();

// 3. Apply filters
let results = nodes
    .filter(|n| filters.matches(n))
    .take(10);
```

### 4. Traverse Outgoing Relationships

```
MATCH (start {id: $id})-[r]->(end)
RETURN type(r) as rel_type, end
```

**HelixDB Implementation:**

```rust
let node = storage.get_node_by_id(id)?;
let edges = storage.outgoing_edges(node.internal_id);
let results: Vec<(RelationType, Node)> = edges
    .map(|e| (e.label.into(), storage.get_node(e.target)))
    .collect();
```

### 5. Traverse with Depth (BFS)

```
MATCH path = (start {id: $id})-[*1..3]->(end)
RETURN path
```

**HelixDB Implementation:**

```rust
fn traverse_bfs(start_id: &str, max_depth: usize) -> Graph {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut graph = Graph::new();

    let start = storage.get_node_by_id(start_id)?;
    queue.push_back((start.internal_id, 0));
    visited.insert(start.internal_id);
    graph.add_node(start);

    while let Some((node_id, depth)) = queue.pop_front() {
        if depth >= max_depth {
            continue;
        }

        for edge in storage.outgoing_edges(node_id) {
            graph.add_edge(edge);

            if !visited.contains(&edge.target) {
                visited.insert(edge.target);
                let target_node = storage.get_node(edge.target);
                graph.add_node(target_node);
                queue.push_back((edge.target, depth + 1));
            }
        }
    }

    graph
}
```

### 6. Find Blocking Chain (Critical Path)

```
MATCH path = (start {id: $id})<-[:BLOCKS*]-(blocker)
WHERE NOT (blocker)<-[:BLOCKS]-()
RETURN path
ORDER BY length(path) DESC
LIMIT 1
```

**HelixDB Implementation:**

```rust
fn find_critical_path(target_id: &str) -> Vec<NodeId> {
    let mut longest_path = vec![];

    fn dfs(node_id: NodeId, path: &mut Vec<NodeId>, longest: &mut Vec<NodeId>) {
        path.push(node_id);

        let blockers: Vec<_> = storage
            .incoming_edges(node_id)
            .filter(|e| e.label == EdgeLabel::Blocks)
            .map(|e| e.source)
            .collect();

        if blockers.is_empty() {
            // Leaf node - check if this is the longest path
            if path.len() > longest.len() {
                *longest = path.clone();
            }
        } else {
            for blocker in blockers {
                dfs(blocker, path, longest);
            }
        }

        path.pop();
    }

    let target = storage.get_node_by_id(target_id)?;
    dfs(target.internal_id, &mut vec![], &mut longest_path);
    longest_path
}
```

### 7. Detect Cycles

```
// Tarjan's algorithm for strongly connected components
CALL algo.scc()
YIELD component
WHERE size(component) > 1
RETURN component
```

**HelixDB Implementation:**

```rust
fn find_cycles(edge_type: EdgeLabel) -> Vec<Vec<NodeId>> {
    // Tarjan's SCC algorithm
    let mut index = 0;
    let mut stack = vec![];
    let mut indices = HashMap::new();
    let mut lowlinks = HashMap::new();
    let mut on_stack = HashSet::new();
    let mut sccs = vec![];

    fn strongconnect(
        v: NodeId,
        // ... parameters
    ) {
        indices.insert(v, index);
        lowlinks.insert(v, index);
        index += 1;
        stack.push(v);
        on_stack.insert(v);

        for edge in storage.outgoing_edges(v).filter(|e| e.label == edge_type) {
            let w = edge.target;
            if !indices.contains_key(&w) {
                strongconnect(w, ...);
                lowlinks.insert(v, min(lowlinks[&v], lowlinks[&w]));
            } else if on_stack.contains(&w) {
                lowlinks.insert(v, min(lowlinks[&v], indices[&w]));
            }
        }

        if lowlinks[&v] == indices[&v] {
            let mut scc = vec![];
            loop {
                let w = stack.pop().unwrap();
                on_stack.remove(&w);
                scc.push(w);
                if w == v { break; }
            }
            if scc.len() > 1 {
                sccs.push(scc);
            }
        }
    }

    for node in storage.all_nodes() {
        if !indices.contains_key(&node.id) {
            strongconnect(node.id, ...);
        }
    }

    sccs
}
```

### 8. Find Related Entities (Union of Relationships)

```
MATCH (n {id: $id})-[r]-(related)
RETURN DISTINCT related, collect(type(r)) as relationships
```

**HelixDB Implementation:**

```rust
fn find_related(id: &str) -> Vec<(Node, Vec<RelationType>)> {
    let node = storage.get_node_by_id(id)?;
    let mut related: HashMap<NodeId, Vec<RelationType>> = HashMap::new();

    // Outgoing edges
    for edge in storage.outgoing_edges(node.internal_id) {
        related
            .entry(edge.target)
            .or_default()
            .push(edge.label.into());
    }

    // Incoming edges
    for edge in storage.incoming_edges(node.internal_id) {
        related
            .entry(edge.source)
            .or_default()
            .push(edge.label.inverse().into());
    }

    related
        .into_iter()
        .map(|(id, rels)| (storage.get_node(id), rels))
        .collect()
}
```

## Data Integrity Constraints

### 1. Unique ID

No two nodes can have the same `id` property.

```rust
// Enforced by unique secondary index
SecondaryIndex::Unique("id")
```

### 2. Valid Relationships

Edges are validated against the validity matrix before creation.

```rust
fn validate_edge(from: &Node, to: &Node, rel_type: RelationType) -> Result<()> {
    let from_type = from.label.entity_type();
    let to_type = to.label.entity_type();

    if !VALIDITY_MATRIX.allows(from_type, to_type, rel_type) {
        return Err(Error::InvalidRelationship {
            from: from.id.clone(),
            to: to.id.clone(),
            reason: format!(
                "{:?} cannot have {:?} relationship to {:?}",
                from_type, rel_type, to_type
            ),
        });
    }
    Ok(())
}
```

### 3. No Self-Loops

An entity cannot have a relationship to itself.

```rust
if from_id == to_id {
    return Err(Error::InvalidRelationship {
        reason: "Self-referential relationships not allowed",
    });
}
```

### 4. Cycle Prevention (Blocking Relationships)

BLOCKS and DEPENDS_ON edges are checked for cycles before creation.

```rust
fn would_create_cycle(from: NodeId, to: NodeId, rel_type: RelationType) -> bool {
    if !rel_type.is_blocking() {
        return false;
    }

    // BFS from `to` to see if we can reach `from`
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(to);

    while let Some(current) = queue.pop_front() {
        if current == from {
            return true;  // Cycle would be created
        }
        if visited.insert(current) {
            for edge in storage.outgoing_edges(current) {
                if edge.label.is_blocking() {
                    queue.push_back(edge.target);
                }
            }
        }
    }

    false
}
```

### 5. Referential Integrity

Edges must point to existing nodes.

```rust
fn create_edge(from_id: &str, to_id: &str, rel_type: RelationType) -> Result<()> {
    let from = storage.get_node_by_id(from_id)
        .ok_or(Error::EntityNotFound { id: from_id })?;
    let to = storage.get_node_by_id(to_id)
        .ok_or(Error::EntityNotFound { id: to_id })?;

    // ... create edge
}
```

### 6. Cascade Delete

When a node is deleted, all its edges are also deleted.

```rust
fn delete_node(id: &str) -> Result<()> {
    let node = storage.get_node_by_id(id)?;

    // Delete outgoing edges
    for edge in storage.outgoing_edges(node.internal_id) {
        storage.delete_edge(edge.id)?;
    }

    // Delete incoming edges
    for edge in storage.incoming_edges(node.internal_id) {
        storage.delete_edge(edge.id)?;
    }

    // Delete vector if exists
    if let Some(vector_id) = node.properties.get("vector_id") {
        storage.vectors.delete(vector_id)?;
    }

    // Delete node
    storage.delete_node(node.internal_id)?;

    Ok(())
}
```

## Schema Evolution

### Adding New Entity Types

1. Add new label to `NodeLabel` enum
2. Define entity-specific properties
3. Update validity matrix
4. Add file storage directory
5. Implement `Entity` trait

### Adding New Relationship Types

1. Add to `EdgeLabel` enum
2. Update validity matrix
3. Document inverse relationship semantics
4. Add CLI support for `helix link`

### Adding New Properties

1. Add to property schema
2. Update markdown parser/serializer
3. Add secondary index if filterable
4. Migrate existing entities (optional defaults)
