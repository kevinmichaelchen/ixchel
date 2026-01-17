//! Integration tests for helix-decisions (specs/tasks.md Task 3.3.1).
//!
//! Run with: cargo test -p helix-decisions --features embeddings-tests --test integration -- --nocapture

#![cfg(feature = "embeddings-tests")]

use helix_config::{EmbeddingConfig, load_shared_config};
use helix_decisions::DecisionSearcher;
use helix_decisions::types::{RelationType, Status};
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;
use tempfile::TempDir;

fn configured_model() -> String {
    load_shared_config()
        .map(|config| config.embedding.model)
        .unwrap_or_else(|_| EmbeddingConfig::default().model)
}

fn model_cache_paths(model_name: &str) -> Vec<PathBuf> {
    let model_dir = format!("models--{}", model_name.replace('/', "--"));
    let mut roots = Vec::new();

    if let Ok(cache_dir) = env::var("FASTEMBED_CACHE_DIR") {
        roots.push(PathBuf::from(cache_dir));
    } else {
        roots.push(PathBuf::from(".fastembed_cache"));
    }

    if let Ok(hf_home) = env::var("HF_HOME") {
        let hf_home = PathBuf::from(hf_home);
        roots.push(hf_home.clone());
        roots.push(hf_home.join("hub"));
    }

    roots
        .into_iter()
        .map(|root| root.join(&model_dir))
        .collect()
}

fn embeddings_available() -> bool {
    let model_name = configured_model();
    let paths = model_cache_paths(&model_name);

    if paths.iter().any(|path| path.exists()) {
        return true;
    }

    let mut message =
        format!("Embeddings tests skipped: model cache not found for {model_name}.\n");
    message.push_str("Looked in:\n");
    for path in &paths {
        message.push_str(&format!("  - {}\n", path.display()));
    }
    message.push_str("\nTo run these tests:\n");
    message.push_str("  1) Download the model (fastembed caches on first use).\n");
    message.push_str(
        "  2) Re-run with: cargo test -p helix-decisions --features embeddings-tests --test integration -- --nocapture\n",
    );
    message.push_str("\nYou can control the cache location via FASTEMBED_CACHE_DIR or HF_HOME.\n");
    eprintln!("{message}");

    false
}

macro_rules! require_embeddings {
    () => {
        if !embeddings_available() {
            return;
        }
    };
}

fn write_decision_file(dir: &Path, id: u32, title: &str, status: &str, body: &str) -> PathBuf {
    write_decision_file_with_relations(dir, id, title, status, body, "")
}

fn write_decision_file_with_relations(
    dir: &Path,
    id: u32,
    title: &str,
    status: &str,
    body: &str,
    relations: &str,
) -> PathBuf {
    let filename = format!("{id:03}-{}.md", title.to_lowercase().replace(' ', "-"));
    let path = dir.join(&filename);

    let content = format!(
        r#"---
id: {id}
title: {title}
status: {status}
date: 2026-01-{id:02}
deciders:
  - Alice
tags:
  - test
{relations}---

{body}
"#,
        id = id,
        title = title,
        status = status,
        relations = relations,
        body = body
    );

    let mut file = fs::File::create(&path).expect("Failed to create decision file");
    file.write_all(content.as_bytes())
        .expect("Failed to write decision file");

    path
}

fn setup_test_env() -> (TempDir, PathBuf) {
    let temp = TempDir::new().expect("Failed to create temp directory");
    let decisions_dir = temp.path().join(".decisions");
    fs::create_dir_all(&decisions_dir).expect("Failed to create .decisions directory");
    (temp, decisions_dir)
}

#[test]
fn scenario_1_initial_indexing_10_decisions() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    for i in 1..=10 {
        write_decision_file(
            &decisions_dir,
            i,
            &format!("Decision {i}"),
            "accepted",
            &format!("This is decision number {i} about architecture."),
        );
    }

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    let stats = searcher.sync(&decisions_dir).expect("Sync failed");

    assert_eq!(stats.scanned, 10, "Should have scanned 10 decisions");
    assert_eq!(stats.added, 10, "Should have added 10 decisions");
    assert_eq!(stats.modified, 0, "Should have 0 modified");
    assert_eq!(stats.deleted, 0, "Should have 0 deleted");
    assert_eq!(stats.unchanged, 0, "Should have 0 unchanged on first sync");

    let response = searcher
        .search("architecture", 10, None, None)
        .expect("Search failed");
    assert!(
        !response.results.is_empty(),
        "Should find results for 'architecture'"
    );
}

#[test]
fn scenario_2_modify_one_decision_delta_detected() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    for i in 1..=5 {
        write_decision_file(
            &decisions_dir,
            i,
            &format!("Decision {i}"),
            "accepted",
            &format!("Original content for decision {i}."),
        );
    }

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    let stats1 = searcher.sync(&decisions_dir).expect("First sync failed");
    assert_eq!(stats1.added, 5);

    write_decision_file(
        &decisions_dir,
        3,
        "Decision 3",
        "accepted",
        "MODIFIED: This decision has been updated with new content.",
    );

    let stats2 = searcher.sync(&decisions_dir).expect("Second sync failed");

    assert_eq!(stats2.scanned, 5, "Should have scanned 5 decisions");
    assert_eq!(stats2.added, 0, "Should have 0 new additions");
    assert_eq!(stats2.modified, 1, "Should have detected 1 modification");
    assert_eq!(stats2.deleted, 0, "Should have 0 deletions");
    assert_eq!(stats2.unchanged, 4, "Should have 4 unchanged");
}

#[test]
fn scenario_3_add_new_decisions_only_new_indexed() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    for i in 1..=3 {
        write_decision_file(
            &decisions_dir,
            i,
            &format!("Initial Decision {i}"),
            "accepted",
            &format!("Content for initial decision {i}."),
        );
    }

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    let stats1 = searcher.sync(&decisions_dir).expect("First sync failed");
    assert_eq!(stats1.added, 3);

    for i in 4..=6 {
        write_decision_file(
            &decisions_dir,
            i,
            &format!("New Decision {i}"),
            "proposed",
            &format!("Content for new decision {i}."),
        );
    }

    let stats2 = searcher.sync(&decisions_dir).expect("Second sync failed");

    assert_eq!(stats2.scanned, 6, "Should have scanned 6 decisions");
    assert_eq!(stats2.added, 3, "Should have added 3 new decisions");
    assert_eq!(stats2.modified, 0, "Should have 0 modified");
    assert_eq!(stats2.deleted, 0, "Should have 0 deleted");
    assert_eq!(stats2.unchanged, 3, "Should have 3 unchanged");
}

#[test]
fn scenario_4_delete_decision_node_and_vector_removed() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    let mut paths = Vec::new();
    for i in 1..=5 {
        let path = write_decision_file(
            &decisions_dir,
            i,
            &format!("Decision {i}"),
            "accepted",
            &format!("Content for decision {i}."),
        );
        paths.push(path);
    }

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    let stats1 = searcher.sync(&decisions_dir).expect("First sync failed");
    assert_eq!(stats1.added, 5);

    fs::remove_file(&paths[2]).expect("Failed to delete file");

    let stats2 = searcher.sync(&decisions_dir).expect("Second sync failed");

    assert_eq!(stats2.scanned, 4, "Should have scanned 4 decisions");
    assert_eq!(stats2.added, 0, "Should have 0 additions");
    assert_eq!(stats2.modified, 0, "Should have 0 modified");
    assert_eq!(stats2.deleted, 1, "Should have deleted 1 decision");
    assert_eq!(stats2.unchanged, 4, "Should have 4 unchanged");

    let response = searcher
        .search("decision", 10, None, None)
        .expect("Search failed");

    for result in &response.results {
        assert_ne!(
            result.id, 3,
            "Deleted decision should not appear in results"
        );
    }
}

#[test]
#[ignore = "Requires embedding model change detection - not yet implemented"]
fn scenario_5_embedding_model_change_reembed_all() {}

#[test]
fn scenario_6_large_repo_delta_under_100ms() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    for i in 1..=100 {
        write_decision_file(
            &decisions_dir,
            i,
            &format!("Performance Decision {i}"),
            "accepted",
            &format!(
                "This is decision {i} with enough content to be meaningful. \
                We need to test performance with realistic content sizes. \
                Architecture decisions often contain context, options, and consequences."
            ),
        );
    }

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    let stats1 = searcher.sync(&decisions_dir).expect("First sync failed");
    assert_eq!(stats1.added, 100);

    let start = Instant::now();
    let stats2 = searcher.sync(&decisions_dir).expect("Delta sync failed");
    let delta_duration = start.elapsed();

    assert_eq!(stats2.unchanged, 100, "All 100 should be unchanged");
    assert_eq!(stats2.added, 0);
    assert_eq!(stats2.modified, 0);
    assert_eq!(stats2.deleted, 0);

    assert!(
        delta_duration.as_millis() < 100,
        "Delta sync took {}ms, expected <100ms",
        delta_duration.as_millis()
    );
}

#[test]
fn scenario_7_chain_traversal_across_supersedes() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    write_decision_file(
        &decisions_dir,
        1,
        "Original Database Choice",
        "superseded",
        "We will use MySQL.",
    );

    write_decision_file_with_relations(
        &decisions_dir,
        2,
        "Updated Database Choice",
        "superseded",
        "We will use PostgreSQL instead of MySQL.",
        "supersedes: 1\n",
    );

    write_decision_file_with_relations(
        &decisions_dir,
        3,
        "Final Database Choice",
        "accepted",
        "We will use CockroachDB for distributed needs.",
        "supersedes: 2\n",
    );

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    searcher.sync(&decisions_dir).expect("Sync failed");

    let chain_response = searcher.get_chain(1).expect("Chain query failed");

    assert_eq!(
        chain_response.chain.len(),
        3,
        "Chain should have 3 decisions"
    );

    assert_eq!(chain_response.chain[0].id, 1, "First in chain should be 1");
    assert_eq!(chain_response.chain[1].id, 2, "Second in chain should be 2");
    assert_eq!(chain_response.chain[2].id, 3, "Third in chain should be 3");

    assert!(
        chain_response.chain[2].is_current,
        "Last node should be marked as current"
    );
}

#[test]
fn scenario_8_related_query_with_all_edge_types() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    write_decision_file(
        &decisions_dir,
        1,
        "API Framework",
        "accepted",
        "We will use Express.js.",
    );

    write_decision_file(
        &decisions_dir,
        2,
        "Old Auth Strategy",
        "superseded",
        "We will use basic auth.",
    );

    write_decision_file(
        &decisions_dir,
        3,
        "Database Schema",
        "accepted",
        "We will use normalized schema.",
    );

    write_decision_file(
        &decisions_dir,
        4,
        "Caching Strategy",
        "accepted",
        "We will use Redis.",
    );

    write_decision_file_with_relations(
        &decisions_dir,
        5,
        "Auth Strategy v2",
        "accepted",
        "We will use JWT with OAuth2.",
        "supersedes: 2\namends: [1]\ndepends_on: 3\nrelated_to: 4\n",
    );

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    searcher.sync(&decisions_dir).expect("Sync failed");

    let related_response = searcher.get_related(5).expect("Related query failed");

    assert_eq!(
        related_response.related.len(),
        4,
        "Should have 4 related decisions"
    );

    let has_supersedes = related_response
        .related
        .iter()
        .any(|r| r.relation == RelationType::Supersedes && r.id == 2);
    let has_amends = related_response
        .related
        .iter()
        .any(|r| r.relation == RelationType::Amends && r.id == 1);
    let has_depends = related_response
        .related
        .iter()
        .any(|r| r.relation == RelationType::DependsOn && r.id == 3);
    let has_related = related_response
        .related
        .iter()
        .any(|r| r.relation == RelationType::RelatedTo && r.id == 4);

    assert!(has_supersedes, "Should have SUPERSEDES relationship to 2");
    assert!(has_amends, "Should have AMENDS relationship to 1");
    assert!(has_depends, "Should have DEPENDS_ON relationship to 3");
    assert!(has_related, "Should have RELATED_TO relationship to 4");
}

#[test]
fn test_search_with_status_filter() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    write_decision_file(
        &decisions_dir,
        1,
        "Proposed Feature",
        "proposed",
        "A proposed architecture feature.",
    );

    write_decision_file(
        &decisions_dir,
        2,
        "Accepted Feature",
        "accepted",
        "An accepted architecture feature.",
    );

    write_decision_file(
        &decisions_dir,
        3,
        "Deprecated Feature",
        "deprecated",
        "A deprecated architecture feature.",
    );

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    searcher.sync(&decisions_dir).expect("Sync failed");

    let response = searcher
        .search("feature", 10, Some(Status::Accepted), None)
        .expect("Search failed");

    assert_eq!(response.results.len(), 1, "Should find 1 accepted result");
    assert_eq!(response.results[0].id, 2);
    assert_eq!(response.results[0].status, Status::Accepted);
}

#[test]
fn test_storage_persistence_across_sessions() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    write_decision_file(
        &decisions_dir,
        1,
        "Persisted Decision",
        "accepted",
        "This decision should persist across sessions.",
    );

    {
        let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
        let stats = searcher.sync(&decisions_dir).expect("Sync failed");
        assert_eq!(stats.added, 1);
    }

    {
        let searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
        let response = searcher
            .search("persist", 10, None, None)
            .expect("Search failed");

        assert!(
            !response.results.is_empty(),
            "Should find persisted decision"
        );
        assert_eq!(response.results[0].id, 1);
    }
}

#[test]
fn test_empty_decisions_directory() {
    require_embeddings!();
    let (temp, decisions_dir) = setup_test_env();

    let mut searcher = DecisionSearcher::new(temp.path()).expect("Failed to create searcher");
    let stats = searcher.sync(&decisions_dir).expect("Sync failed");

    assert_eq!(stats.scanned, 0);
    assert_eq!(stats.added, 0);
    assert_eq!(stats.unchanged, 0);
}
