//! adr-search CLI
//!
//! Semantic search over Architecture Decision Records.

use adr_search::{ADRSearcher, Status};
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Semantic search over Architecture Decision Records.
#[derive(Parser, Debug)]
#[command(name = "adr-search")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Search query.
    query: String,

    /// Directory containing ADRs.
    #[arg(short, long, default_value = ".decisions")]
    directory: PathBuf,

    /// Maximum number of results.
    #[arg(short, long, default_value = "10")]
    limit: usize,

    /// Filter by status (proposed, accepted, superseded, deprecated).
    #[arg(long)]
    status: Option<String>,

    /// Filter by tags (comma-separated).
    #[arg(long)]
    tags: Option<String>,

    /// Output JSON format.
    #[arg(short, long)]
    json: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Parse filters
    let status_filter = cli
        .status
        .map(|s| s.parse::<Status>())
        .transpose()
        .map_err(|e| anyhow::anyhow!(e))?;
    let tags_filter = cli
        .tags
        .map(|t| t.split(',').map(str::trim).map(String::from).collect());

    // Create searcher and sync
    let mut searcher = ADRSearcher::new()?;
    searcher.sync(&cli.directory)?;

    // Search
    let response = searcher.search(&cli.query, cli.limit, status_filter, tags_filter)?;

    // Output
    if cli.json {
        println!("{}", serde_json::to_string_pretty(&response)?);
    } else {
        print_pretty(&response);
    }

    // Exit code based on results
    if response.results.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}

fn print_pretty(response: &adr_search::SearchResponse) {
    if response.results.is_empty() {
        println!("No results found for: \"{}\"", response.query);
        return;
    }

    println!();
    println!("Query: \"{}\"", response.query);
    println!("Found: {} results", response.count);
    println!();

    for (i, result) in response.results.iter().enumerate() {
        println!("[{}] {:03}: {}", i + 1, result.id, result.title);
        println!("    Status: {} | Score: {:.2}", result.status, result.score);
        if !result.tags.is_empty() {
            println!("    Tags: {}", result.tags.join(", "));
        }
        println!(
            "    Date: {} | Deciders: {}",
            result.date,
            result.deciders.join(", ")
        );
        println!("    File: {}", result.file_path.display());
        println!();
    }
}
