use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    #[default]
    Open,
    InProgress,
    Blocked,
    Closed,
}

impl Status {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::InProgress => "in_progress",
            Self::Blocked => "blocked",
            Self::Closed => "closed",
        }
    }

    pub const fn is_open(self) -> bool {
        !matches!(self, Self::Closed)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "open" => Ok(Self::Open),
            "in_progress" | "in-progress" | "inprogress" => Ok(Self::InProgress),
            "blocked" => Ok(Self::Blocked),
            "closed" => Ok(Self::Closed),
            _ => Err(format!("invalid status: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum IssueType {
    Bug,
    Feature,
    #[default]
    Task,
    Epic,
    Chore,
}

impl IssueType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bug => "bug",
            Self::Feature => "feature",
            Self::Task => "task",
            Self::Epic => "epic",
            Self::Chore => "chore",
        }
    }
}

impl std::fmt::Display for IssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for IssueType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bug" => Ok(Self::Bug),
            "feature" => Ok(Self::Feature),
            "task" => Ok(Self::Task),
            "epic" => Ok(Self::Epic),
            "chore" => Ok(Self::Chore),
            _ => Err(format!("invalid issue type: {s}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum Priority {
    Critical = 0,
    High = 1,
    #[default]
    Medium = 2,
    Low = 3,
    Backlog = 4,
}

impl Priority {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_u8(n: u8) -> Option<Self> {
        match n {
            0 => Some(Self::Critical),
            1 => Some(Self::High),
            2 => Some(Self::Medium),
            3 => Some(Self::Low),
            4 => Some(Self::Backlog),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Backlog => "backlog",
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u8())
    }
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" | "critical" => Ok(Self::Critical),
            "1" | "high" => Ok(Self::High),
            "2" | "medium" => Ok(Self::Medium),
            "3" | "low" => Ok(Self::Low),
            "4" | "backlog" => Ok(Self::Backlog),
            _ => Err(format!("invalid priority: {s} (expected 0-4)")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum CreatorType {
    #[default]
    Human,
    Agent,
}

impl CreatorType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Human => "human",
            Self::Agent => "agent",
        }
    }
}

impl std::fmt::Display for CreatorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DepType {
    #[default]
    Blocks,
    Related,
    WaitsFor,
}

impl DepType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocks => "blocks",
            Self::Related => "related",
            Self::WaitsFor => "waits_for",
        }
    }
}

impl std::fmt::Display for DepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for DepType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "blocks" => Ok(Self::Blocks),
            "related" => Ok(Self::Related),
            "waits_for" | "waits-for" | "waitsfor" => Ok(Self::WaitsFor),
            _ => Err(format!("invalid dependency type: {s}")),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub id: String,
    pub dep_type: DepType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub created_by_type: CreatorType,
}

impl Comment {
    pub fn new(
        body: impl Into<String>,
        created_by: impl Into<String>,
        creator_type: CreatorType,
    ) -> Self {
        Self {
            id: crate::id::generate_comment_id(),
            body: body.into(),
            created_at: Utc::now(),
            created_by: created_by.into(),
            created_by_type: creator_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub body: String,
    pub status: Status,
    pub priority: Priority,
    pub issue_type: IssueType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<DateTime<Utc>>,
    pub created_by: String,
    pub created_by_type: CreatorType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub depends_on: Vec<Dependency>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<Comment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_minutes: Option<i32>,
    #[serde(skip)]
    pub content_hash: Option<String>,
}

impl Issue {
    pub fn builder(title: impl Into<String>) -> IssueBuilder {
        IssueBuilder::new(title)
    }

    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    pub fn close(&mut self, reason: Option<String>, closed_by: &str, creator_type: CreatorType) {
        self.status = Status::Closed;
        self.closed_at = Some(Utc::now());
        self.touch();

        if let Some(reason) = reason {
            self.comments.push(Comment::new(
                format!("Closed: {reason}"),
                closed_by,
                creator_type,
            ));
        }
    }

    pub fn reopen(&mut self) {
        self.status = Status::Open;
        self.closed_at = None;
        self.touch();
    }

    pub fn add_comment(
        &mut self,
        body: impl Into<String>,
        author: &str,
        creator_type: CreatorType,
    ) {
        self.comments.push(Comment::new(body, author, creator_type));
        self.touch();
    }

    pub fn is_blocked_by(&self, other_id: &str) -> bool {
        self.depends_on
            .iter()
            .any(|d| d.id == other_id && d.dep_type == DepType::Blocks)
    }

    pub fn add_dependency(&mut self, dep_id: impl Into<String>, dep_type: DepType) {
        let dep_id = dep_id.into();
        if !self.depends_on.iter().any(|d| d.id == dep_id) {
            self.depends_on.push(Dependency {
                id: dep_id,
                dep_type,
            });
            self.touch();
        }
    }

    pub fn remove_dependency(&mut self, dep_id: &str) -> bool {
        let len_before = self.depends_on.len();
        self.depends_on.retain(|d| d.id != dep_id);
        let removed = self.depends_on.len() < len_before;
        if removed {
            self.touch();
        }
        removed
    }
}

#[derive(Debug, Default)]
pub struct IssueBuilder {
    title: String,
    body: String,
    issue_type: IssueType,
    priority: Priority,
    created_by: String,
    created_by_type: CreatorType,
    assignee: Option<String>,
    agent_id: Option<String>,
    session_id: Option<String>,
    parent_id: Option<String>,
    labels: Vec<String>,
    external_ref: Option<String>,
    estimated_minutes: Option<i32>,
}

impl IssueBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            created_by: whoami::username(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = body.into();
        self
    }

    #[must_use]
    pub const fn issue_type(mut self, t: IssueType) -> Self {
        self.issue_type = t;
        self
    }

    #[must_use]
    pub const fn priority(mut self, p: Priority) -> Self {
        self.priority = p;
        self
    }

    #[must_use]
    pub fn created_by(mut self, by: impl Into<String>, creator_type: CreatorType) -> Self {
        self.created_by = by.into();
        self.created_by_type = creator_type;
        self
    }

    #[must_use]
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    #[must_use]
    pub fn agent(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into());
        self.created_by_type = CreatorType::Agent;
        self
    }

    #[must_use]
    pub fn session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    #[must_use]
    pub fn parent(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    #[must_use]
    pub fn labels(mut self, labels: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.labels = labels.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn external_ref(mut self, r: impl Into<String>) -> Self {
        self.external_ref = Some(r.into());
        self
    }

    #[must_use]
    pub const fn estimated_minutes(mut self, m: i32) -> Self {
        self.estimated_minutes = Some(m);
        self
    }

    pub fn build(self) -> Issue {
        let now = Utc::now();
        Issue {
            id: crate::id::generate_issue_id(),
            title: self.title,
            body: self.body,
            status: Status::Open,
            priority: self.priority,
            issue_type: self.issue_type,
            created_at: now,
            updated_at: now,
            closed_at: None,
            created_by: self.created_by,
            created_by_type: self.created_by_type,
            assignee: self.assignee,
            agent_id: self.agent_id,
            session_id: self.session_id,
            external_ref: self.external_ref,
            parent_id: self.parent_id,
            labels: self.labels,
            depends_on: Vec::new(),
            comments: Vec::new(),
            estimated_minutes: self.estimated_minutes,
            content_hash: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
