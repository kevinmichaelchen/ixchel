use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum HbdError {
    #[error("issue not found: {0}")]
    IssueNotFound(String),

    #[error("ambiguous issue ID '{0}' matches multiple issues: {}", .1.join(", "))]
    AmbiguousId(String, Vec<String>),

    #[error("cycle detected: {}", .0.join(" -> "))]
    CycleDetected(Vec<String>),

    #[error("invalid issue format in {path}: {reason}")]
    InvalidFormat { path: PathBuf, reason: String },

    #[error("invalid YAML frontmatter: {0}")]
    InvalidFrontmatter(String),

    #[error("project not initialized (run 'hbd init' first)")]
    NotInitialized,

    #[error("project already initialized at {0}")]
    AlreadyInitialized(PathBuf),

    #[error("sync conflict: {0}")]
    SyncConflict(String),

    #[error("database error: {0}")]
    Database(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("{0}")]
    Other(String),
}

impl HbdError {
    pub const fn exit_code(&self) -> i32 {
        match self {
            Self::IssueNotFound(_) | Self::AmbiguousId(_, _) => 3,
            Self::CycleDetected(_) => 4,
            Self::SyncConflict(_) => 5,
            Self::NotInitialized
            | Self::AlreadyInitialized(_)
            | Self::InvalidFormat { .. }
            | Self::InvalidFrontmatter(_) => 2,
            Self::Database(_) => 6,
            Self::Io(_) | Self::Yaml(_) | Self::Json(_) | Self::Other(_) => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, HbdError>;
