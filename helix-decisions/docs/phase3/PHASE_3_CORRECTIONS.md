# PHASE 3: HelixDB Integration - CRITICAL CORRECTIONS

**Status:** Plan identified 10 critical/high issues requiring API realignment  
**Impact:** Runtime failures without fixes  
**Severity:** High - affects all graph operations  

---

## Summary of Issues

| Issue | Severity | Impact | Status |
|-------|----------|--------|--------|
| Edge insertion missing adjacency indexes | Critical | Chain/related queries fail | ✓ Fixed |
| Vector insertion API mismatch | High | Embeddings not stored | ✓ Fixed |
| Node/edge construction skips arena/helpers | High | Serialization fails | ✓ Fixed |
| Metadata key collision risk | Medium | Version conflicts | ✓ Fixed |
| Config path not plumbed | Medium | DB location ignored | ✓ Fixed |
| Decision ID lookups need secondary indices | Medium | Property queries fail | ✓ Fixed |
| Label hashing for adjacency keys | High | Traversal fails | ✓ Fixed |
| Traversal ops vs direct writes | Design | Performance/complexity tradeoff | ✓ Decided |
| Vector-ID ↔ node-ID mapping missing | High | Can't retrieve embeddings | ✓ Fixed |
| Deletion doesn't handle vectors | High | Orphaned vectors, memory leak | ✓ Fixed |

---

## 1. ISSUE: Edge Insertion Missing Adjacency Indexes

### ❌ Current Plan (BROKEN)
```rust
// Only writes edges_db, missing adjacency!
edges_db.put(&mut wtxn, &edge_id, &edge.to_bincode_bytes()?)?;
// Chain/related queries won't find this edge
```

### ✅ CORRECTED

Edges must be written to **THREE** databases:

```rust
// When creating an edge from node A to node B with label "SUPERSEDES"

let edge_id = Uuid::new_v4().as_u128();
let edge = Edge {
    id: edge_id,
    label: "SUPERSEDES",  // Must be allocated in arena
    version: 1,
    from_node: node_a_id,
    to_node: node_b_id,
    properties: None,
};

// 1. Write edge data
edges_db.put(&mut wtxn, &HelixGraphStorage::edge_key(&edge_id), &edge.to_bincode_bytes()?)?;

// 2. Write outgoing adjacency (from_node + label_hash → edge_id + to_node)
let label_hash = hash_label(edge.label, None);
let out_key = HelixGraphStorage::out_edge_key(&node_a_id, &label_hash);
let out_value = pack_edge_data(edge_id, node_b_id);
out_edges_db.put(&mut wtxn, &out_key, &out_value)?;

// 3. Write incoming adjacency (to_node + label_hash → edge_id + from_node)
let in_key = HelixGraphStorage::in_edge_key(&node_b_id, &label_hash);
let in_value = pack_edge_data(edge_id, node_a_id);
in_edges_db.put(&mut wtxn, &in_key, &in_value)?;
```

### Impact on Plan
- **helix_backend.rs**: `create_relationship_edges()` must write 3 DBs, not 1
- **Testing**: Must verify all 3 writes, test traversal queries
- **Performance**: Slightly slower per-edge, but necessary for traversal correctness

### References
- `helix-db/src/helix_engine/storage_core/mod.rs` (out_edge_key, in_edge_key, pack_edge_data helpers)
- `helix-db/src/helix_engine/tests/storage_tests.rs` (canonical examples)

---

## 2. ISSUE: Node/Edge Construction Skips Arena & Helpers

### ❌ Current Plan (BROKEN)
```rust
// Direct construction, missing arena, missing key helpers
let node = Node {
    id: node_id,
    label: "DECISION",
    version: 1,
    properties: serialize_decision_properties(decision),  // ← Not arena-allocated!
};
nodes_db.put(&mut wtxn, &node_id, &node.to_bincode_bytes()?)?;  // ← Wrong key!
```

### ✅ CORRECTED

Use arena allocation and storage key helpers:

```rust
use bumpalo::Bump;
use helix_db::utils::items::Node;
use helix_db::utils::properties::ImmutablePropertiesMap;

// Transaction setup
let mut wtxn = engine.storage.graph_env.write_txn()?;
let arena = Bump::new();

// 1. Create arena-allocated properties map
let mut props = Vec::new();
props.push(("id", Value::I64(decision.metadata.id as i64)));
props.push(("title", Value::String(arena.alloc(decision.metadata.title.clone()))));
props.push(("status", Value::String(arena.alloc(decision.metadata.status.to_string()))));
props.push(("file_path", Value::String(arena.alloc(decision.file_path.to_string_lossy().to_string()))));
// ... more properties ...
let properties = ImmutablePropertiesMap::from_items(props, &arena)?;

// 2. Create node with arena-allocated label
let label = arena.alloc("DECISION");
let node = Node {
    id: node_id,
    label,
    version: 1,
    properties: Some(properties),
};

// 3. Use storage key helper
let key = HelixGraphStorage::node_key(&node_id);
nodes_db.put(&mut wtxn, &key, &node.to_bincode_bytes()?)?;

// Same for edges
let edge_label = arena.alloc("SUPERSEDES");
let edge = Edge {
    id: edge_id,
    label: edge_label,
    version: 1,
    from_node: node_a_id,
    to_node: node_b_id,
    properties: None,
};
let edge_key = HelixGraphStorage::edge_key(&edge_id);
edges_db.put(&mut wtxn, &edge_key, &edge.to_bincode_bytes()?)?;
```

### Impact on Plan
- **helix_backend.rs**: Must allocate arena in `sync()` and each mutation
- **manifest.rs**: No changes needed
- **Testing**: Must verify properties round-trip correctly

### References
- `helix-db/src/utils/items.rs` (Node<'arena>, Edge<'arena>)
- `helix-db/src/utils/properties.rs` (ImmutablePropertiesMap)
- `helix-db/src/helix_engine/storage_core/mod.rs` (node_key, edge_key helpers)

---

## 3. ISSUE: Vector Insertion API Mismatch

### ❌ Current Plan (BROKEN)
```rust
// HNSW::insert doesn't exist; API expects different signature
self.engine.storage.vectors.add_vector(&mut wtxn, node_id, embedding)?;
// This method doesn't exist!
```

### ✅ CORRECTED - TWO DESIGN CHOICES

#### Option A (RECOMMENDED): Vector ID in Node Properties + Secondary Index

**Approach:** Store vector_id as node property, search vectors independently, map results.

```rust
// 1. Insert vector (HNSW generates its own UUID)
let vector_id = Uuid::new_v4().as_u128();
let embedding_f64: Vec<f64> = embedding.iter().map(|&x| x as f64).collect();
self.engine.storage.vectors.insert(&mut wtxn, vector_id, &embedding_f64)?;

// 2. Store vector_id in node properties
let mut props = vec![
    ("id", Value::I64(decision.metadata.id as i64)),
    ("vector_id", Value::String(arena.alloc(vector_id.to_string()))),
    // ... other properties ...
];
let properties = ImmutablePropertiesMap::from_items(props, &arena)?;

// 3. When searching: search vectors → get vector_ids → lookup nodes via secondary index
let search_results = self.engine.storage.vectors.search(&embedding_f64, limit)?;
for result in search_results {
    // result.id is the vector_id, need to map back to node_id
    let node_id = lookup_node_by_vector_id(result.id)?;  // Via secondary index
}
```

**Pros:**
- ✅ Separation of concerns (vectors independent from graph)
- ✅ Can delete vectors without node deletion
- ✅ Aligns with Helix's architecture

**Cons:**
- ⚠ Extra lookup step (vector_id → node_id)
- ⚠ Need to maintain secondary index on vector_id property

#### Option B: Metadata Mapping (Fallback)

```rust
// Store mapping in metadata DB
let mapping = vec![(vector_id, node_id)];
metadata_db.put(&mut wtxn, b"vector:mapping", bincode::serialize(&mapping)?)?;

// When searching, load mapping and translate
let mapping: Vec<(u128, u128)> = bincode::deserialize(metadata_db.get(&rtxn, b"vector:mapping")?)?;
```

**Pros:**
- ✅ Simpler, no secondary index needed
- ✅ All mappings in one place

**Cons:**
- ⚠ Must load entire mapping for every search
- ⚠ Scales poorly (O(n) lookups)
- ⚠ Not how Helix is designed

### **RECOMMENDATION: Use Option A (vector_id property + secondary index)**

### Impact on Plan
- **helix_backend.rs**: 
  - `upsert_decision_node()`: Store vector_id in properties
  - `search()`: Must map vector results back to node_ids via secondary index
  - Add secondary index creation on vector_id
- **manifest.rs**: Store vector_id alongside node_id
- **Testing**: Test vector round-trip and result mapping

### References
- `helix-db/src/helix_engine/vector_core/hnsw.rs` (insert API)
- `helix-db/src/helix_engine/vector_core/vector_core.rs` (VectorCore methods)
- `helix-db/src/helix_engine/vector_core/vector.rs` (vector insertion details)

---

## 4. ISSUE: Vector Deletion (Orphaned Vectors)

### ❌ Current Plan (BROKEN)
```rust
// Only deletes node, vector is orphaned
storage.drop_node(&mut txn, &node_id)?;
// Vector still exists but unreachable
```

### ✅ CORRECTED

```rust
// When deleting a decision node:

// 1. Get vector_id from node properties before deleting
let node = storage.get_node(&rtxn, &node_id, &arena)?;
let vector_id = node.get_property("vector_id")
    .and_then(|v| v.as_string())
    .and_then(|s| u128::from_str(s).ok())?;

// 2. Delete node (drops node + edges + indices)
storage.drop_node(&mut txn, &node_id)?;

// 3. Tombstone vector
if let Some(vid) = vector_id {
    storage.drop_vector(&mut txn, &vid)?;
}

txn.commit()?;
```

### Impact on Plan
- **helix_backend.rs**: `delete_decision_node()` must handle vector cleanup
- **Testing**: Verify vectors are tombstoned, not leaked

### References
- `helix-db/src/helix_engine/storage_core/storage_methods.rs` (drop_node, drop_vector)

---

## 5. ISSUE: Label Hashing for Adjacency Keys

### ❌ Current Plan (IGNORED)
```rust
// Missing label hash calculation
out_edges_db.put(&mut wtxn, &out_key, &out_value)?;
// What is out_key? Needs label_hash!
```

### ✅ CORRECTED

```rust
use helix_db::utils::label_hash::hash_label;

let label_hash = hash_label(edge.label, None);
let out_key = {
    let mut key = Vec::new();
    key.extend_from_slice(&node_a_id.to_le_bytes());
    key.extend_from_slice(&label_hash.to_le_bytes());
    key
};
```

### Impact on Plan
- **helix_backend.rs**: `create_relationship_edges()` must hash labels
- **Testing**: Verify hashes are consistent

### References
- `helix-db/src/utils/label_hash.rs`

---

## 6. ISSUE: Secondary Indices (Decision ID Lookups)

### ❌ Current Plan (BROKEN)
```rust
// Plan assumes you can lookup decisions by id: u32
let decision = get_decision_by_id(1)?;  // This won't work!
// No secondary index created
```

### ✅ CORRECTED

Create secondary index on `id` property:

```rust
// In backend initialization
pub fn new(repo_root: &Path) -> Result<Self> {
    // ... create engine ...
    
    // Create secondary index on decision id
    self.engine.storage.create_secondary_index("decision_id")?;
    
    Ok(Self { engine, ... })
}

// When inserting decision node with id property
let props = vec![
    ("id", Value::I64(decision.metadata.id as i64)),
    // ... other properties ...
];

// Secondary index is maintained automatically if in config
```

### Impact on Plan
- **helix_backend.rs**: 
  - Create secondary indices in `new()`
  - Mark indices to maintain in config
- **Testing**: Verify index lookups work

### References
- `helix-db/src/helix_engine/storage_core/mod.rs` (create_secondary_index)
- `helix-db/src/helix_engine/traversal_core/ops/source/add_n.rs` (index usage)

---

## 7. ISSUE: Metadata Key Namespace Collision

### ❌ Current Plan (RISKY)
```rust
// Just use "manifest:decisions" as key
// But metadata already has "storage_version", "vector_endianness", etc.
metadata_db.put(&mut wtxn, b"manifest:decisions", manifest_bytes)?;
```

### ✅ CORRECTED

Use explicit versioned namespace:

```rust
// Define manifest key constant
const MANIFEST_KEY: &str = "manifest:helix-decisions:v1";

// When saving
metadata_db.put(
    &mut wtxn,
    MANIFEST_KEY.as_bytes(),
    &serde_json::to_vec(&manifest)?
)?;

// Document key in comments
// MANIFEST_KEY format: "manifest:{app}:v{version}"
// Existing keys: "storage_version", "vector_endianness"
```

### Impact on Plan
- **manifest.rs**: Define MANIFEST_KEY constant, add comments
- **Testing**: No changes

### References
- `helix-db/src/helix_engine/storage_core/metadata.rs`

---

## 8. ISSUE: Config Path Not Plumbed

### ❌ Current Plan (BROKEN)
```rust
// Plan mentions HELIX_DB_PATH env var
// But Helix reads HelixGraphEngineOpts.path, not env vars
```

### ✅ CORRECTED

```rust
// In helix_backend.rs
pub fn new(repo_root: &Path) -> Result<Self> {
    // Determine DB path
    let db_path = std::env::var("HELIX_DB_PATH")
        .unwrap_or_else(|_| {
            repo_root.join(".helixdb").to_string_lossy().to_string()
        });
    
    // Create engine options and PASS PATH EXPLICITLY
    let opts = HelixGraphEngineOpts {
        path: db_path,
        config: Config::default(),
        version_info: VersionInfo::default(),
    };
    
    let engine = HelixGraphEngine::new(opts)?;
    Ok(Self { engine, ... })
}
```

### Impact on Plan
- **helix_backend.rs**: `new()` must plumb path through HelixGraphEngineOpts
- **Testing**: Test with custom HELIX_DB_PATH

### References
- `helix-db/src/helix_engine/traversal_core/mod.rs` (HelixGraphEngineOpts)
- `helix-container/src/main.rs` (example of path handling)

---

## 9. DESIGN CHOICE: Traversal Ops vs Direct Storage Writes

### ❌ Current Plan (MIXES BOTH)
```rust
// Plan uses direct DB writes (edges_db.put, nodes_db.put)
// But also relies on traversal queries (get_chain, get_related)
// Inconsistent approach!
```

### ✅ DECISION: USE DIRECT STORAGE WRITES

**Rationale:**
- ✅ You control all writes (adjacency, vectors, indices)
- ✅ Simpler for incremental indexing (batch operations)
- ✅ No arena context overhead per-write
- ⚠ You must maintain adjacency indexes manually (3 DBs per edge)

**Alternative:** Traversal ops (add_n, add_e) would handle secondary indices automatically, but requires arena management and is overkill for this use case.

### Impact on Plan
- **helix_backend.rs**: Stick with direct storage writes, update all 3 DBs per edge
- **manifest.rs**: No changes
- **Testing**: Verify adjacency manually in tests

### References
- `helix-db/src/helix_engine/traversal_core/ops/source/add_n.rs` (if you ever reconsider)

---

## 10. ISSUE: Node ID Strategy

### ❌ Current Plan (CONFUSING)
```rust
// Plan uses decision_id: u32 as local ID
// But also node_id: u128 as HelixDB node ID
// Unclear relationship, storage confusion
```

### ✅ CORRECTED

**Keep two IDs, use consistently:**

```rust
// In node properties
let props = vec![
    ("id", Value::I64(decision.metadata.id as i64)),  // Local sequential ID (1,2,3...)
    ("node_id", Value::String(arena.alloc(node_id.to_string()))),  // UUID for Helix
];

// In manifest
#[derive(Serialize, Deserialize)]
pub struct ManifestEntry {
    pub file_path: String,
    pub mtime: u64,
    pub size: u64,
    pub content_hash: String,
    pub node_id: u128,              // ← HelixDB node ID (UUID)
    pub vector_id: u128,            // ← Vector ID (UUID)
    pub embedding_model: String,
    pub indexer_version: String,
}

// Node IDs: always UUID v4
use uuid::Uuid;
let node_id = Uuid::new_v4().as_u128();
```

### Impact on Plan
- **manifest.rs**: Add vector_id field
- **helix_backend.rs**: Use UUID v4 consistently for node_id
- **Testing**: No changes

### References
- `helix-db/src/utils/id.rs`

---

## CORRECTED ARCHITECTURE SUMMARY

### Modified Graph Schema

**Node (DECISION):**
```
id: u32 (1, 2, 3...) [PROPERTY]
title: string [PROPERTY]
status: string [PROPERTY]
date: string [PROPERTY]
file_path: string [PROPERTY]
content_hash: string [PROPERTY]
tags: [string] [PROPERTY]
deciders: [string] [PROPERTY]
vector_id: u128 [PROPERTY] ← NEW: Link to embedding

node_id: u128 [KEY] ← UUID, not property
```

**Edges (3 DB writes per edge):**
```
edges_db: edge_id → edge_bytes
out_edges_db: (from_id || label_hash) → (edge_id || to_id)
in_edges_db: (to_id || label_hash) → (edge_id || from_id)
```

**Manifest (in metadata):**
```
Key: "manifest:helix-decisions:v1"
Value: {
  "entries": {
    ".decisions/001-arch.md": {
      "mtime": 1704585600,
      "size": 2048,
      "content_hash": "abc123...",
      "node_id": "550e8400-e29b-41d4-a716...",
      "vector_id": "660e8400-e29b-41d4-a716...",
      "embedding_model": "BAAI/bge-small-en-v1.5",
      "indexer_version": "1"
    }
  }
}
```

---

## CORRECTED MODULE SPECIFICATIONS

### manifest.rs (UPDATED)
```diff
  pub struct ManifestEntry {
      pub file_path: String,
      pub mtime: u64,
      pub size: u64,
      pub content_hash: String,
      pub node_id: u128,
+     pub vector_id: u128,  ← NEW
      pub embedding_model: String,
      pub indexer_version: String,
  }

+ pub const MANIFEST_KEY: &str = "manifest:helix-decisions:v1";
```

### helix_backend.rs (MAJOR CHANGES)

#### In upsert_decision_node():
```diff
- // Just create node and vector separately
+ // 1. Allocate arena
+ let arena = Bump::new();
+ 
+ // 2. Create and insert vector first (get vector_id)
+ let embedding_f64 = embedding.iter().map(|&x| x as f64).collect();
+ let vector_id = Uuid::new_v4().as_u128();
+ self.engine.storage.vectors.insert(&mut wtxn, vector_id, &embedding_f64)?;
+
+ // 3. Create node with vector_id property
+ let props = vec![
+     ("id", Value::I64(decision.metadata.id as i64)),
+     ("vector_id", Value::String(arena.alloc(vector_id.to_string()))),
+     // ... other properties ...
+ ];
+
+ // 4. Store node using key helper
+ let node_key = HelixGraphStorage::node_key(&node_id);
```

#### In create_relationship_edges():
```diff
- // Write edge to edges_db only
+ // 1. Write to edges_db
+ let edge_key = HelixGraphStorage::edge_key(&edge_id);
+ edges_db.put(&mut wtxn, &edge_key, &edge.to_bincode_bytes()?)?;
+
+ // 2. Write outgoing adjacency
+ let label_hash = hash_label(edge.label, None);
+ let out_key = { /* node_id || label_hash */ };
+ out_edges_db.put(&mut wtxn, &out_key, &pack_edge_data(edge_id, to_node))?;
+
+ // 3. Write incoming adjacency
+ let in_key = { /* to_node || label_hash */ };
+ in_edges_db.put(&mut wtxn, &in_key, &pack_edge_data(edge_id, from_node))?;
```

#### In delete_decision_node():
```diff
- // Delete node only
+ // 1. Get vector_id before deletion
+ let node = storage.get_node(&rtxn, &node_id, &arena)?;
+ let vector_id = extract_vector_id_from_node(&node);
+
+ // 2. Delete node (drops edges + indices)
+ storage.drop_node(&mut wtxn, &node_id)?;
+
+ // 3. Tombstone vector
+ if let Some(vid) = vector_id {
+     storage.drop_vector(&mut wtxn, &vid)?;
+ }
```

#### In sync():
```diff
- // Delegate to backend
+ // Create arena at start of sync
+ let arena = Bump::new();
+
+ // For each file change:
+ // - Allocate labels/properties in arena
+ // - Write 3 DBs per edge
+ // - Handle vector insertion separately
```

### storage.rs (MINIMAL CHANGES)
- ✅ No changes needed (trait remains same)
- Update imports to use corrected helix_backend

---

## CORRECTED TESTING STRATEGY

### manifest.rs Tests (6)
- Serialization/deserialization ✓
- CRUD (get, upsert, remove) ✓
- Vector ID field handling ✓
- Metadata persistence ✓
- Key namespace ✓

### git_utils.rs Tests (4)
- Git listing ✓
- Directory walk fallback ✓
- .gitignore respect ✓

### helix_backend.rs Tests (20+) ← EXPANDED
- **Node creation (4 tests):**
  - ✓ Arena allocation
  - ✓ Properties round-trip
  - ✓ Key helper usage
  - ✓ Property lookup
  
- **Vector handling (4 tests):**
  - ✓ Insert f32→f64 conversion
  - ✓ Vector ID stored in properties
  - ✓ Delete tombstones vector
  - ✓ Search result mapping
  
- **Edge creation (6 tests):**
  - ✓ Write all 3 DBs
  - ✓ Label hashing
  - ✓ Adjacency keys correct
  - ✓ Traversal can find edges
  - ✓ Both directions (out/in)
  
- **Incremental indexing (3 stages, 6 tests):**
  - ✓ Stage 1: Stat skip
  - ✓ Stage 2: Hash skip
  - ✓ Stage 3: Full re-index
  - ✓ Vector deletion
  - ✓ Secondary index maintenance
  
- **Queries (3 tests):**
  - ✓ Vector search + mapping
  - ✓ Chain traversal via out_edges_db
  - ✓ Related via relationship edges

### storage.rs Tests (5)
- Trait implementation ✓
- Search filter ✓
- Chain query ✓
- Related query ✓
- Secondary index usage ✓

**Total: 35+ tests (up from 30)**

---

## PRIORITY FIX CHECKLIST

### Critical (Must Fix Before Implementation)
- [x] Edge writes: Add out_edges_db + in_edges_db writes
- [x] Node construction: Use arena + ImmutablePropertiesMap
- [x] Vector insertion: API mismatch, need vector_id mapping
- [x] Label hashing: Calculate and use in adjacency keys
- [x] Node key helpers: Use HelixGraphStorage::node_key/edge_key

### High Priority
- [x] Vector deletion: Tombstone vectors on node delete
- [x] Secondary indices: Create and maintain for decision_id lookups
- [x] Config path: Plumb HELIX_DB_PATH through HelixGraphEngineOpts

### Medium Priority
- [x] Metadata key: Use "manifest:helix-decisions:v1" namespace
- [x] Traversal ops decision: Decided on direct storage writes
- [x] Test expansion: 35+ tests to cover all HelixDB APIs

---

## CHANGED FILES (REVISED)

### Create (New Files)
| File | Changes from Original |
|------|----------------------|
| `helix-decisions/src/manifest.rs` | Add vector_id field, add MANIFEST_KEY constant |
| `helix-decisions/src/git_utils.rs` | No changes |
| `helix-decisions/src/helix_backend.rs` | **MAJOR REWRITE**: Arena allocation, 3 DB writes per edge, vector ID mapping, secondary indices |

### Modify (Existing Files)
| File | Changes from Original |
|------|----------------------|
| `helix-decisions/src/storage.rs` | No changes needed |
| `helix-decisions/src/lib.rs` | No changes needed |
| `helix-decisions/Cargo.toml` | No changes needed |

---

## RISK REASSESSMENT

| Risk | Mitigation | Status |
|------|-----------|--------|
| Edge traversals fail | Write all 3 DBs per edge | ✓ Fixed |
| Vectors not stored | Create vector_id mapping | ✓ Fixed |
| Serialization errors | Use arena + helpers | ✓ Fixed |
| Config ignored | Plumb path through opts | ✓ Fixed |
| Property lookups slow | Create secondary indices | ✓ Fixed |
| Orphaned vectors | Delete both node + vector | ✓ Fixed |
| Key collisions | Use versioned namespace | ✓ Fixed |

---

## NEXT STEPS

1. **Review this document** - Understand all corrections
2. **Update PHASE_3_PLAN.md** - Incorporate all corrections
3. **Start Session 1** - Implement with corrected API usage
4. **Test against HelixDB directly** - Don't assume API behavior
5. **Reference storage_tests.rs** - Use as canonical examples

All corrections are **API-driven**, not design-driven. The architecture remains sound; only HelixDB-specific details were wrong.

