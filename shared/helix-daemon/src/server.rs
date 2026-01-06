use crate::{
    Command, DaemonError, ErrorCode, PROTOCOL_VERSION, PingResponse, Request, Response,
    ResponsePayload, ShutdownResponse, StatusPayload, StatusResponse,
};
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixListener;
use tokio::sync::{RwLock, broadcast};

const MAX_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB limit

pub struct Server {
    socket_path: String,
    start_time: std::time::Instant,
    shutdown_tx: broadcast::Sender<()>,
    state: Arc<RwLock<ServerState>>,
}

#[derive(Default)]
struct ServerState {
    request_count: u64,
}

impl Server {
    pub fn new(socket_path: impl Into<String>) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            socket_path: socket_path.into(),
            start_time: std::time::Instant::now(),
            shutdown_tx,
            state: Arc::new(RwLock::new(ServerState::default())),
        }
    }

    pub fn expanded_socket_path(&self) -> String {
        expand_tilde(&self.socket_path)
    }

    pub async fn run(&self) -> Result<(), DaemonError> {
        let socket_path = self.expanded_socket_path();

        if let Some(parent) = Path::new(&socket_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        if Path::new(&socket_path).exists() {
            tokio::fs::remove_file(&socket_path).await?;
        }

        let listener = UnixListener::bind(&socket_path)?;
        tracing::info!("helixd listening on {}", socket_path);

        let mut shutdown_rx = self.shutdown_tx.subscribe();

        loop {
            tokio::select! {
                accept_result = listener.accept() => {
                    match accept_result {
                        Ok((stream, _)) => {
                            let state = Arc::clone(&self.state);
                            let start_time = self.start_time;
                            let shutdown_tx = self.shutdown_tx.clone();
                            tokio::spawn(async move {
                                if let Err(e) = handle_connection(stream, state, start_time, shutdown_tx).await {
                                    tracing::error!("Connection error: {}", e);
                                }
                            });
                        }
                        Err(e) => {
                            tracing::error!("Accept error: {}", e);
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("Shutdown signal received");
                    break;
                }
            }
        }

        let _ = tokio::fs::remove_file(&socket_path).await;
        Ok(())
    }

    pub fn shutdown(&self) {
        let _ = self.shutdown_tx.send(());
    }
}

async fn handle_connection(
    stream: tokio::net::UnixStream,
    state: Arc<RwLock<ServerState>>,
    start_time: std::time::Instant,
    shutdown_tx: broadcast::Sender<()>,
) -> Result<(), DaemonError> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            break;
        }

        if line.len() > MAX_MESSAGE_SIZE {
            let resp = Response::error("", ErrorCode::InvalidRequest, "Message too large");
            let json = serde_json::to_string(&resp)?;
            writer.write_all(json.as_bytes()).await?;
            writer.write_all(b"\n").await?;
            continue;
        }

        let response = match serde_json::from_str::<Request>(line.trim()) {
            Ok(req) => {
                {
                    let mut s = state.write().await;
                    s.request_count += 1;
                }

                if req.version == PROTOCOL_VERSION {
                    handle_command(&req, start_time, &shutdown_tx)
                } else {
                    Response::error(
                        &req.id,
                        ErrorCode::IncompatibleVersion,
                        format!(
                            "Protocol version mismatch: expected {PROTOCOL_VERSION}, got {}",
                            req.version
                        ),
                    )
                }
            }
            Err(e) => Response::error("", ErrorCode::InvalidRequest, e.to_string()),
        };

        let json = serde_json::to_string(&response)?;
        writer.write_all(json.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        writer.flush().await?;
    }

    Ok(())
}

fn handle_command(
    req: &Request,
    start_time: std::time::Instant,
    shutdown_tx: &broadcast::Sender<()>,
) -> Response {
    match &req.command {
        Command::Ping => Response::ok(
            &req.id,
            ResponsePayload::Ping(PingResponse {
                daemon_version: env!("CARGO_PKG_VERSION").to_string(),
            }),
        ),

        Command::EnqueueSync(_payload) => Response::error(
            &req.id,
            ErrorCode::InternalError,
            "enqueue_sync not yet implemented (Phase 3)",
        ),

        Command::WaitSync(_payload) => Response::error(
            &req.id,
            ErrorCode::InternalError,
            "wait_sync not yet implemented (Phase 3)",
        ),

        Command::Status(StatusPayload { .. }) => {
            #[allow(clippy::cast_possible_truncation)]
            let uptime_ms = start_time.elapsed().as_millis() as u64;
            Response::ok(
                &req.id,
                ResponsePayload::Status(StatusResponse {
                    queues: vec![],
                    uptime_ms,
                }),
            )
        }

        Command::Shutdown(payload) => {
            tracing::info!("Shutdown requested: {}", payload.reason);
            let _ = shutdown_tx.send(());
            Response::ok(&req.id, ResponsePayload::Shutdown(ShutdownResponse {}))
        }
    }
}

fn expand_tilde(path: &str) -> String {
    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = dirs_next::home_dir()
    {
        return home.join(rest).to_string_lossy().to_string();
    }
    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let expanded = expand_tilde("~/.helix/run/helixd.sock");
        assert!(!expanded.starts_with('~'));
        assert!(expanded.contains(".helix/run/helixd.sock"));
    }

    #[test]
    fn test_expand_tilde_no_tilde() {
        let path = "/tmp/test.sock";
        assert_eq!(expand_tilde(path), path);
    }
}
