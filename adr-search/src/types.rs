//! Core types for ADR search.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// ADR status values.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Under discussion, not yet decided.
    Proposed,
    /// Decision has been made and is in effect.
    Accepted,
    /// Replaced by another ADR.
    Superseded,
    /// No longer relevant or applicable.
    Deprecated,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Proposed => write!(f, "proposed"),
            Self::Accepted => write!(f, "accepted"),
            Self::Superseded => write!(f, "superseded"),
            Self::Deprecated => write!(f, "deprecated"),
        }
    }
}

impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(Self::Proposed),
            "accepted" => Ok(Self::Accepted),
            "superseded" => Ok(Self::Superseded),
            "deprecated" => Ok(Self::Deprecated),
            _ => Err(format!("Invalid status: {s}")),
        }
    }
}

/// ADR metadata from YAML frontmatter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ADRMetadata {
    /// Unique identifier (sequential).
    pub id: u32,
    /// Short title of the decision.
    pub title: String,
    /// Current status.
    pub status: Status,
    /// Date the decision was made.
    pub date: NaiveDate,
    /// People involved in the decision.
    #[serde(default)]
    pub deciders: Vec<String>,
    /// Topic tags.
    #[serde(default)]
    pub tags: Vec<String>,
    /// ID of ADR this supersedes.
    #[serde(default)]
    pub supersedes: Option<u32>,
    /// ID of ADR that supersedes this.
    #[serde(default)]
    pub superseded_by: Option<u32>,
}

/// Full ADR with body and computed fields.
#[derive(Debug, Clone)]
pub struct ADR {
    /// Parsed metadata from frontmatter.
    pub metadata: ADRMetadata,
    /// Markdown body content (without frontmatter).
    pub body: String,
    /// Path to the source file.
    pub file_path: PathBuf,
    /// SHA256 hash of file content (for delta detection).
    pub content_hash: String,
    /// Embedding vector (populated after embedding).
    pub embedding: Option<Vec<f32>>,
}

/// Search result with relevance score.
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    /// ADR ID.
    pub id: u32,
    /// ADR title.
    pub title: String,
    /// ADR status.
    pub status: Status,
    /// Relevance score (0.0 to 1.0).
    pub score: f32,
    /// Topic tags.
    pub tags: Vec<String>,
    /// Decision date.
    pub date: NaiveDate,
    /// People involved.
    pub deciders: Vec<String>,
    /// Path to source file.
    pub file_path: PathBuf,
}

/// Search response envelope.
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    /// Original query string.
    pub query: String,
    /// Number of results.
    pub count: usize,
    /// Ranked results.
    pub results: Vec<SearchResult>,
}
