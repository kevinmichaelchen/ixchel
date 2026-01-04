use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::error::{HbdError, Result};
use crate::id::matches_partial;
use crate::markdown;
use crate::types::Issue;

const TICKETS_DIR: &str = ".tickets";
const HELIX_DIR: &str = ".helix";
const CONFIG_FILE: &str = "config.toml";

pub struct TicketStore {
    root: PathBuf,
}

impl TicketStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn from_current_dir() -> Result<Self> {
        let root = find_project_root()?;
        Ok(Self::new(root))
    }

    pub fn tickets_dir(&self) -> PathBuf {
        self.root.join(TICKETS_DIR)
    }

    pub fn helix_dir(&self) -> PathBuf {
        self.root.join(HELIX_DIR)
    }

    pub fn config_path(&self) -> PathBuf {
        self.helix_dir().join(CONFIG_FILE)
    }

    pub fn is_initialized(&self) -> bool {
        self.tickets_dir().exists()
    }

    pub fn init(&self) -> Result<()> {
        if self.is_initialized() {
            return Err(HbdError::AlreadyInitialized(self.root.clone()));
        }

        fs::create_dir_all(self.tickets_dir())?;
        fs::create_dir_all(self.helix_dir())?;

        let config = default_config();
        fs::write(self.config_path(), config)?;

        self.update_gitignore()?;

        Ok(())
    }

    fn update_gitignore(&self) -> Result<()> {
        let gitignore_path = self.root.join(".gitignore");
        let helix_db_entry = ".helix/helix.db/";
        let models_entry = ".helix/models/";

        let content = if gitignore_path.exists() {
            fs::read_to_string(&gitignore_path)?
        } else {
            String::new()
        };

        let mut lines: Vec<&str> = content.lines().collect();
        let original_len = lines.len();

        if !lines.iter().any(|l| l.trim() == helix_db_entry) {
            lines.push(helix_db_entry);
        }
        if !lines.iter().any(|l| l.trim() == models_entry) {
            lines.push(models_entry);
        }

        if lines.len() > original_len {
            let mut new_content = lines.join("\n");
            if !new_content.ends_with('\n') {
                new_content.push('\n');
            }
            fs::write(gitignore_path, new_content)?;
        }

        Ok(())
    }

    pub fn issue_path(&self, id: &str) -> PathBuf {
        self.tickets_dir().join(format!("{id}.md"))
    }

    pub fn read_issue(&self, id: &str) -> Result<Issue> {
        let path = self.issue_path(id);
        if !path.exists() {
            return Err(HbdError::IssueNotFound(id.to_string()));
        }
        let content = fs::read_to_string(&path)?;
        markdown::parse_issue(&content, &path)
    }

    pub fn write_issue(&self, issue: &Issue) -> Result<()> {
        if !self.is_initialized() {
            return Err(HbdError::NotInitialized);
        }

        let path = self.issue_path(&issue.id);
        let content = markdown::render_issue(issue);
        fs::write(path, content)?;
        Ok(())
    }

    pub fn delete_issue(&self, id: &str) -> Result<()> {
        let path = self.issue_path(id);
        if !path.exists() {
            return Err(HbdError::IssueNotFound(id.to_string()));
        }
        fs::remove_file(path)?;
        Ok(())
    }

    pub fn list_issue_ids(&self) -> Result<Vec<String>> {
        if !self.is_initialized() {
            return Err(HbdError::NotInitialized);
        }

        let mut ids = Vec::new();
        for entry in fs::read_dir(self.tickets_dir())? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md")
                && let Some(stem) = path.file_stem()
            {
                ids.push(stem.to_string_lossy().into_owned());
            }
        }
        ids.sort();
        Ok(ids)
    }

    pub fn read_all_issues(&self) -> Result<Vec<Issue>> {
        let ids = self.list_issue_ids()?;
        let mut issues = Vec::with_capacity(ids.len());
        for id in ids {
            match self.read_issue(&id) {
                Ok(issue) => issues.push(issue),
                Err(e) => {
                    tracing::warn!("skipping malformed issue {id}: {e}");
                }
            }
        }
        Ok(issues)
    }

    pub fn read_all_issues_map(&self) -> Result<HashMap<String, Issue>> {
        let issues = self.read_all_issues()?;
        Ok(issues.into_iter().map(|i| (i.id.clone(), i)).collect())
    }

    pub fn resolve_id(&self, partial: &str) -> Result<String> {
        let ids = self.list_issue_ids()?;
        let matches: Vec<_> = ids
            .iter()
            .filter(|id| matches_partial(id, partial))
            .collect();

        match matches.len() {
            0 => Err(HbdError::IssueNotFound(partial.to_string())),
            1 => Ok(matches[0].clone()),
            _ => Err(HbdError::AmbiguousId(
                partial.to_string(),
                matches.into_iter().cloned().collect(),
            )),
        }
    }

    pub fn exists(&self, id: &str) -> bool {
        self.issue_path(id).exists()
    }
}

fn find_project_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join(TICKETS_DIR).exists() || current.join(HELIX_DIR).exists() {
            return Ok(current);
        }
        if !current.pop() {
            return Ok(std::env::current_dir()?);
        }
    }
}

const fn default_config() -> &'static str {
    r#"# hbd configuration
# See https://github.com/kevinmichaelchen/helix-tools for documentation

[project]
name = ""
default_assignee = ""

[sync]
auto_sync = true
interval_seconds = 5

[embeddings]
backend = "fastembed"
model = "BGESmallENV15"
offline_only = true

[search]
default_limit = 20
"#
}
