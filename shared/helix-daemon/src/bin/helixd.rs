use clap::Parser;
use helix_daemon::{DEFAULT_SOCKET_PATH, Server};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "helixd", about = "Helix-tools daemon for IPC and sync")]
struct Args {
    #[arg(long, default_value = DEFAULT_SOCKET_PATH)]
    socket: String,
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let args = Args::parse();
    let server = Server::new(&args.socket);

    tracing::info!("Starting helixd with socket: {}", args.socket);

    if let Err(e) = server.run().await {
        tracing::error!("Server error: {}", e);
        return ExitCode::from(1);
    }

    ExitCode::SUCCESS
}
