use std::process::ExitCode;

use clap::{Parser, Subcommand};
use hbd::{CreatorType, Issue, IssueType, Priority, Status, TicketStore};

#[derive(Parser)]
#[command(name = "hbd")]
#[command(author, version, about = "Git-first issue tracker powered by HelixDB")]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,

    Info,

    Create {
        title: String,

        #[arg(short, long)]
        description: Option<String>,

        #[arg(short = 't', long, default_value = "task")]
        r#type: String,

        #[arg(short, long, default_value = "2")]
        priority: u8,

        #[arg(short, long)]
        labels: Option<String>,

        #[arg(short, long)]
        assignee: Option<String>,

        #[arg(long)]
        agent: Option<String>,

        #[arg(long)]
        session: Option<String>,

        #[arg(long)]
        parent: Option<String>,

        #[arg(long)]
        ephemeral: bool,
    },

    Show {
        id: String,
    },

    List {
        #[arg(short, long)]
        status: Option<String>,

        #[arg(short = 't', long)]
        r#type: Option<String>,

        #[arg(short, long)]
        priority: Option<u8>,

        #[arg(short, long)]
        label: Option<String>,

        #[arg(short, long)]
        assignee: Option<String>,

        #[arg(long)]
        project: Option<String>,

        #[arg(long)]
        include_ephemeral: bool,
    },

    Update {
        id: String,

        #[arg(long)]
        status: Option<String>,

        #[arg(long)]
        priority: Option<u8>,

        #[arg(long)]
        title: Option<String>,

        #[arg(long)]
        assignee: Option<String>,
    },

    Close {
        id: String,

        #[arg(short, long)]
        reason: Option<String>,

        #[arg(long)]
        force: bool,
    },

    Reopen {
        id: String,
    },

    Search {
        query: String,

        #[arg(long)]
        semantic: bool,

        #[arg(long)]
        hybrid: bool,

        #[arg(short, long, default_value = "20")]
        limit: usize,
    },

    Similar {
        id: String,

        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    Dep {
        #[command(subcommand)]
        command: DepCommands,
    },

    Label {
        #[command(subcommand)]
        command: LabelCommands,
    },

    Comment {
        id: String,
        message: String,

        #[arg(long)]
        agent: Option<String>,
    },

    Comments {
        id: String,
    },

    Ready {
        #[arg(long)]
        project: Option<String>,
    },

    Blocked {
        #[arg(long)]
        project: Option<String>,
    },

    Explain {
        id: String,
    },

    CriticalPath {
        id: String,
    },

    Graph {
        id: String,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(long, default_value = "5")]
        depth: usize,
    },

    Stale {
        #[arg(long, default_value = "14")]
        days: u32,

        #[arg(short, long)]
        status: Option<String>,

        #[arg(short, long)]
        limit: Option<usize>,
    },

    Count {
        #[arg(short, long)]
        status: Option<String>,

        #[arg(short = 't', long)]
        r#type: Option<String>,
    },

    Merge {
        sources: Vec<String>,

        #[arg(long)]
        into: String,

        #[arg(long)]
        dry_run: bool,
    },

    Restore {
        id: String,
    },

    Admin {
        #[command(subcommand)]
        command: AdminCommands,
    },

    Sync {
        #[arg(long)]
        import_only: bool,

        #[arg(long)]
        export_only: bool,
    },

    Health {
        #[arg(long)]
        label: Option<String>,
    },

    Stats {
        #[arg(long)]
        project: Option<String>,
    },

    Context {
        #[arg(long)]
        query: Option<String>,

        #[arg(long)]
        limit: Option<usize>,
    },

    Compact {
        #[arg(long)]
        dry_run: bool,
    },

    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    Daemon {
        #[command(subcommand)]
        command: DaemonCommands,
    },
}

#[derive(Subcommand)]
enum DepCommands {
    Add {
        from: String,
        dep_type: String,
        to: String,
    },
    Remove {
        from: String,
        dep_type: String,
        to: String,
    },
    List {
        id: String,
    },
    Cycles,
}

#[derive(Subcommand)]
enum LabelCommands {
    Add { id: String, label: String },
    Remove { id: String, label: String },
    List { id: String },
    ListAll,
}

#[derive(Subcommand)]
enum ConfigCommands {
    Show,
    Set { key: String, value: String },
}

#[derive(Subcommand)]
enum AdminCommands {
    Cleanup {
        #[arg(long, default_value = "90")]
        older_than: u32,

        #[arg(long)]
        dry_run: bool,

        #[arg(long)]
        force: bool,

        #[arg(long)]
        cascade: bool,
    },
}

#[derive(Subcommand, Clone, Copy)]
enum DaemonCommands {
    Start,
    Stop,
    Status,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            ExitCode::from(e.exit_code() as u8)
        }
    }
}

fn run(cli: Cli) -> hbd::Result<()> {
    match cli.command {
        Commands::Init => cmd_init(),
        Commands::Info => cmd_info(cli.json),
        Commands::Create {
            title,
            description,
            r#type,
            priority,
            labels,
            assignee,
            agent,
            session,
            parent,
            ephemeral: _,
        } => cmd_create(
            &title,
            description.as_deref(),
            &r#type,
            priority,
            labels.as_deref(),
            assignee.as_deref(),
            agent.as_deref(),
            session.as_deref(),
            parent.as_deref(),
            cli.json,
        ),
        Commands::Show { id } => cmd_show(&id, cli.json),
        Commands::List {
            status,
            r#type,
            priority,
            label: _,
            assignee,
            project: _,
            include_ephemeral: _,
        } => cmd_list(
            status.as_deref(),
            r#type.as_deref(),
            priority,
            assignee.as_deref(),
            cli.json,
        ),
        Commands::Update {
            id,
            status,
            priority,
            title,
            assignee,
        } => cmd_update(
            &id,
            status.as_deref(),
            priority,
            title.as_deref(),
            assignee.as_deref(),
            cli.json,
        ),
        Commands::Close {
            id,
            reason,
            force: _,
        } => cmd_close(&id, reason.as_deref(), cli.json),
        Commands::Reopen { id } => cmd_reopen(&id, cli.json),
        Commands::Comment { id, message, agent } => {
            cmd_comment(&id, &message, agent.as_deref(), cli.json)
        }
        Commands::Comments { id } => cmd_comments(&id, cli.json),
        Commands::Ready { project: _ } => cmd_ready(cli.json),
        Commands::Blocked { project: _ } => cmd_blocked(cli.json),
        _ => {
            eprintln!("Command not yet implemented. See specs/tasks.md for roadmap.");
            Ok(())
        }
    }
}

fn cmd_init() -> hbd::Result<()> {
    let store = TicketStore::new(std::env::current_dir()?);
    store.init()?;
    println!("Initialized hbd in current directory");
    println!("  Created .tickets/");
    println!("  Created .helix/config.toml");
    println!("  Updated .gitignore");
    Ok(())
}

fn cmd_info(json: bool) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let ids = store.list_issue_ids()?;
    let open_count = ids
        .iter()
        .filter_map(|id| store.read_issue(id).ok())
        .filter(|i| i.status != Status::Closed)
        .count();

    if json {
        let info = serde_json::json!({
            "initialized": store.is_initialized(),
            "tickets_dir": store.tickets_dir(),
            "total_issues": ids.len(),
            "open_issues": open_count,
        });
        println!("{}", serde_json::to_string_pretty(&info)?);
    } else {
        println!("hbd status:");
        println!("  Initialized: {}", store.is_initialized());
        println!("  Tickets dir: {}", store.tickets_dir().display());
        println!("  Total issues: {}", ids.len());
        println!("  Open issues: {open_count}");
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn cmd_create(
    title: &str,
    description: Option<&str>,
    issue_type: &str,
    priority: u8,
    labels: Option<&str>,
    assignee: Option<&str>,
    agent: Option<&str>,
    session: Option<&str>,
    parent: Option<&str>,
    json: bool,
) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;

    let issue_type: IssueType = issue_type.parse().map_err(hbd::HbdError::Other)?;
    let priority = Priority::from_u8(priority).unwrap_or(Priority::Medium);

    let mut builder = Issue::builder(title)
        .issue_type(issue_type)
        .priority(priority);

    if let Some(desc) = description {
        builder = builder.body(desc);
    }
    if let Some(a) = assignee {
        builder = builder.assignee(a);
    }
    if let Some(agent_id) = agent {
        builder = builder.agent(agent_id);
    }
    if let Some(sess) = session {
        builder = builder.session(sess);
    }
    if let Some(p) = parent {
        let parent_id = store.resolve_id(p)?;
        builder = builder.parent(parent_id);
    }
    if let Some(l) = labels {
        builder = builder.labels(l.split(',').map(str::trim));
    }

    let issue = builder.build();
    store.write_issue(&issue)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issue)?);
    } else {
        println!("{}", issue.id);
    }

    Ok(())
}

fn cmd_show(id: &str, json: bool) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let id = store.resolve_id(id)?;
    let issue = store.read_issue(&id)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issue)?);
    } else {
        println!("{} {}", issue.id, issue.title);
        println!(
            "Status: {} | Priority: {} | Type: {}",
            issue.status,
            issue.priority.as_u8(),
            issue.issue_type
        );
        if let Some(ref assignee) = issue.assignee {
            println!("Assignee: {assignee}");
        }
        if let Some(ref parent) = issue.parent_id {
            println!("Parent: {parent}");
        }
        if !issue.labels.is_empty() {
            println!("Labels: {}", issue.labels.join(", "));
        }
        if !issue.depends_on.is_empty() {
            println!("Depends on:");
            for dep in &issue.depends_on {
                println!("  - {} ({})", dep.id, dep.dep_type);
            }
        }
        if !issue.body.is_empty() {
            println!("\n{}", issue.body);
        }
        if !issue.comments.is_empty() {
            println!("\nComments ({}):", issue.comments.len());
            for c in &issue.comments {
                println!(
                    "  [{} — {}]",
                    c.created_at.format("%Y-%m-%d %H:%M"),
                    c.created_by
                );
                for line in c.body.lines().take(2) {
                    println!("    {line}");
                }
            }
        }
    }
    Ok(())
}

fn cmd_list(
    status: Option<&str>,
    issue_type: Option<&str>,
    priority: Option<u8>,
    assignee: Option<&str>,
    json: bool,
) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let mut issues = store.read_all_issues()?;

    if let Some(s) = status {
        let s: Status = s.parse().map_err(hbd::HbdError::Other)?;
        issues.retain(|i| i.status == s);
    }
    if let Some(t) = issue_type {
        let t: IssueType = t.parse().map_err(hbd::HbdError::Other)?;
        issues.retain(|i| i.issue_type == t);
    }
    if let Some(p) = priority
        && let Some(p) = Priority::from_u8(p)
    {
        issues.retain(|i| i.priority == p);
    }
    if let Some(a) = assignee {
        issues.retain(|i| i.assignee.as_deref() == Some(a));
    }

    issues.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| b.created_at.cmp(&a.created_at))
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&issues)?);
    } else {
        if issues.is_empty() {
            println!("No issues found.");
            return Ok(());
        }
        println!(
            "{:<12} {:<4} {:<12} {:<10} Title",
            "ID", "P", "Status", "Type"
        );
        println!("{}", "-".repeat(60));
        for i in &issues {
            println!(
                "{:<12} {:<4} {:<12} {:<10} {}",
                i.id,
                i.priority.as_u8(),
                i.status,
                i.issue_type,
                truncate(&i.title, 40)
            );
        }
        println!("\n{} issue(s)", issues.len());
    }
    Ok(())
}

fn cmd_update(
    id: &str,
    status: Option<&str>,
    priority: Option<u8>,
    title: Option<&str>,
    assignee: Option<&str>,
    json: bool,
) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let id = store.resolve_id(id)?;
    let mut issue = store.read_issue(&id)?;

    if let Some(s) = status {
        issue.status = s.parse().map_err(hbd::HbdError::Other)?;
    }
    if let Some(p) = priority
        && let Some(p) = Priority::from_u8(p)
    {
        issue.priority = p;
    }
    if let Some(t) = title {
        issue.title = t.to_string();
    }
    if let Some(a) = assignee {
        issue.assignee = Some(a.to_string());
    }

    issue.touch();
    store.write_issue(&issue)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issue)?);
    } else {
        println!("Updated {}", issue.id);
    }
    Ok(())
}

fn cmd_close(id: &str, reason: Option<&str>, json: bool) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let id = store.resolve_id(id)?;
    let mut issue = store.read_issue(&id)?;

    let user = whoami::username();
    issue.close(reason.map(String::from), &user, CreatorType::Human);
    store.write_issue(&issue)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issue)?);
    } else {
        println!("Closed {}", issue.id);
    }
    Ok(())
}

fn cmd_reopen(id: &str, json: bool) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let id = store.resolve_id(id)?;
    let mut issue = store.read_issue(&id)?;

    issue.reopen();
    store.write_issue(&issue)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issue)?);
    } else {
        println!("Reopened {}", issue.id);
    }
    Ok(())
}

fn cmd_comment(id: &str, message: &str, agent: Option<&str>, json: bool) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let id = store.resolve_id(id)?;
    let mut issue = store.read_issue(&id)?;

    let (author, creator_type) = agent.map_or_else(
        || (whoami::username(), CreatorType::Human),
        |agent_id| (agent_id.to_string(), CreatorType::Agent),
    );

    issue.add_comment(message, &author, creator_type);
    store.write_issue(&issue)?;

    if json {
        let comment = issue.comments.last().unwrap();
        println!("{}", serde_json::to_string_pretty(comment)?);
    } else {
        println!("Added comment to {}", issue.id);
    }
    Ok(())
}

fn cmd_comments(id: &str, json: bool) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let id = store.resolve_id(id)?;
    let issue = store.read_issue(&id)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&issue.comments)?);
    } else {
        if issue.comments.is_empty() {
            println!("No comments on {}", issue.id);
            return Ok(());
        }
        for c in &issue.comments {
            let suffix = if c.created_by_type == CreatorType::Agent {
                " (agent)"
            } else {
                ""
            };
            println!(
                "### {} — {}{}",
                c.created_at.format("%Y-%m-%d %H:%M:%S"),
                c.created_by,
                suffix
            );
            println!("{}\n", c.body);
        }
    }
    Ok(())
}

fn cmd_ready(json: bool) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let issues = store.read_all_issues()?;
    let issues_map = store.read_all_issues_map()?;

    let ready: Vec<_> = issues
        .into_iter()
        .filter(|i| i.status == Status::Open)
        .filter(|i| {
            i.depends_on.iter().all(|dep| {
                issues_map
                    .get(&dep.id)
                    .is_none_or(|blocker| blocker.status == Status::Closed)
            })
        })
        .collect();

    let mut ready = ready;
    ready.sort_by(|a, b| {
        a.priority
            .cmp(&b.priority)
            .then_with(|| b.created_at.cmp(&a.created_at))
    });

    if json {
        println!("{}", serde_json::to_string_pretty(&ready)?);
    } else {
        if ready.is_empty() {
            println!("No ready issues.");
            return Ok(());
        }
        println!("Ready issues (no open blockers):\n");
        println!("{:<12} {:<4} {:<10} Title", "ID", "P", "Type");
        println!("{}", "-".repeat(50));
        for i in &ready {
            println!(
                "{:<12} {:<4} {:<10} {}",
                i.id,
                i.priority.as_u8(),
                i.issue_type,
                truncate(&i.title, 35)
            );
        }
        println!("\n{} issue(s) ready", ready.len());
    }
    Ok(())
}

fn cmd_blocked(json: bool) -> hbd::Result<()> {
    let store = TicketStore::from_current_dir()?;
    let issues = store.read_all_issues()?;
    let issues_map = store.read_all_issues_map()?;

    let blocked: Vec<_> = issues
        .into_iter()
        .filter(|i| i.status != Status::Closed)
        .filter(|i| {
            i.depends_on.iter().any(|dep| {
                issues_map
                    .get(&dep.id)
                    .is_some_and(|blocker| blocker.status != Status::Closed)
            })
        })
        .collect();

    if json {
        let result: Vec<_> = blocked
            .iter()
            .map(|i| {
                let blockers: Vec<_> = i
                    .depends_on
                    .iter()
                    .filter_map(|d| issues_map.get(&d.id))
                    .filter(|b| b.status != Status::Closed)
                    .map(|b| serde_json::json!({"id": b.id, "title": b.title, "status": b.status.as_str()}))
                    .collect();
                serde_json::json!({
                    "issue": {"id": i.id, "title": i.title},
                    "blocked_by": blockers
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        if blocked.is_empty() {
            println!("No blocked issues.");
            return Ok(());
        }
        println!("Blocked issues:\n");
        for i in &blocked {
            println!("{} {}", i.id, truncate(&i.title, 40));
            for dep in &i.depends_on {
                if let Some(blocker) = issues_map.get(&dep.id)
                    && blocker.status != Status::Closed
                {
                    println!("  blocked by: {} ({})", blocker.id, blocker.status);
                }
            }
            println!();
        }
    }
    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}...", &s[..max - 3])
    }
}
