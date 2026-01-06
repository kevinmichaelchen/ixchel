use thiserror::Error;

/// Errors that can occur in helix-repo operations
#[derive(Debug, Error)]
pub enum Error {
    /// Invalid repository URL format
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Repository not found in local cache
    #[error("Repository not found: {0}")]
    NotFound(String),

    /// Repository already exists at the target path
    #[error("Repository already exists: {}", .0.display())]
    AlreadyExists(std::path::PathBuf),

    /// Git command failed
    #[error("Git command failed with exit code {0:?}")]
    GitFailed(Option<i32>),

    /// IO error during file operations
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),
}

impl Error {
    /// Get the exit code for this error type
    #[must_use]
    pub const fn exit_code(&self) -> i32 {
        match self {
            Self::InvalidUrl(_) => 1,
            Self::NotFound(_) => 2,
            Self::AlreadyExists(_) => 3,
            Self::GitFailed(_) => 4,
            Self::Io(_) => 10,
            Self::Config(_) => 11,
        }
    }
}
