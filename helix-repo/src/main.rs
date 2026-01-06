use clap::Parser;

/// Manage git repository clones for helix-tools
#[derive(Parser, Debug)]
#[command(name = "helix-repo")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Clone a repository
    Clone {
        /// Repository URL
        url: String,

        /// Shallow clone (depth 1)
        #[arg(long)]
        shallow: bool,

        /// Clone specific branch
        #[arg(long)]
        branch: Option<String>,

        /// Print what would be done without executing
        #[arg(long)]
        dry_run: bool,
    },

    /// List cloned repositories
    List {
        /// Filter repositories by pattern
        #[arg(long)]
        filter: Option<String>,
    },

    /// Show repository information
    Info {
        /// Repository name (owner/repo format)
        name: String,
    },

    /// Remove a cloned repository
    Remove {
        /// Repository name (owner/repo format)
        name: String,

        /// Print what would be done without executing
        #[arg(long)]
        dry_run: bool,
    },

    /// Print the root directory path
    Root,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Clone {
            url,
            shallow,
            branch,
            dry_run,
        } => {
            println!(
                "Clone: {} (shallow={}, branch={:?}, dry_run={})",
                url, shallow, branch, dry_run
            );
            // TODO: Implement
        }
        Commands::List { filter } => {
            println!("List: filter={:?}", filter);
            // TODO: Implement
        }
        Commands::Info { name } => {
            println!("Info: {}", name);
            // TODO: Implement
        }
        Commands::Remove { name, dry_run } => {
            println!("Remove: {} (dry_run={})", name, dry_run);
            // TODO: Implement
        }
        Commands::Root => {
            println!("~/.cache/helix/repos");
            // TODO: Implement with actual config loading
        }
    }
}
