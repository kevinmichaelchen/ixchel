use crate::{
    Command, DEFAULT_SOCKET_PATH, DaemonError, EnqueueSyncPayload, Request, Response,
    ResponseResult, SyncState, WaitSyncPayload,
};
use std::path::Path;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

const CONNECT_RETRY_DELAY_MS: u64 = 100;
const MAX_CONNECT_RETRIES: u32 = 5;

pub struct Client {
    socket_path: String,
}

impl Client {
    pub fn new() -> Self {
        Self::with_socket_path(DEFAULT_SOCKET_PATH)
    }

    pub fn with_socket_path(path: impl Into<String>) -> Self {
        Self {
            socket_path: path.into(),
        }
    }

    fn expanded_socket_path(&self) -> String {
        expand_tilde(&self.socket_path)
    }

    pub async fn connect(&self) -> Result<UnixStream, DaemonError> {
        let socket_path = self.expanded_socket_path();

        for attempt in 0..MAX_CONNECT_RETRIES {
            match UnixStream::connect(&socket_path).await {
                Ok(stream) => return Ok(stream),
                Err(e)
                    if e.kind() == std::io::ErrorKind::NotFound
                        || e.kind() == std::io::ErrorKind::ConnectionRefused =>
                {
                    if attempt == 0 {
                        self.start_daemon().await?;
                    }
                    tokio::time::sleep(tokio::time::Duration::from_millis(
                        CONNECT_RETRY_DELAY_MS * (u64::from(attempt) + 1),
                    ))
                    .await;
                }
                Err(e) => return Err(e.into()),
            }
        }

        Err(DaemonError::Internal(format!(
            "Failed to connect to daemon after {MAX_CONNECT_RETRIES} retries"
        )))
    }

    async fn start_daemon(&self) -> Result<(), DaemonError> {
        let socket_path = self.expanded_socket_path();

        if let Some(parent) = Path::new(&socket_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let ixcheld_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("ixcheld")))
            .filter(|p| p.exists())
            .unwrap_or_else(|| "ixcheld".into());

        tokio::process::Command::new(ixcheld_path)
            .arg("--socket")
            .arg(&socket_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| DaemonError::Internal(format!("Failed to start ixcheld: {e}")))?;

        Ok(())
    }

    pub async fn send(&self, request: Request) -> Result<Response, DaemonError> {
        let mut stream = self.connect().await?;

        let json = serde_json::to_string(&request)?;
        stream.write_all(json.as_bytes()).await?;
        stream.write_all(b"\n").await?;
        stream.flush().await?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).await?;

        let response: Response = serde_json::from_str(line.trim())?;
        Ok(response)
    }

    pub async fn ping(&self) -> Result<String, DaemonError> {
        let request = Request::new("", "", Command::Ping);
        let response = self.send(request).await?;

        match response.result {
            ResponseResult::Ok { payload } => {
                if let crate::ResponsePayload::Ping(ping) = payload {
                    Ok(ping.daemon_version)
                } else {
                    Err(DaemonError::Internal("Unexpected response type".into()))
                }
            }
            ResponseResult::Error { error } => Err(DaemonError::Internal(error.message)),
        }
    }

    pub async fn enqueue_sync(
        &self,
        repo_root: &str,
        tool: &str,
        directory: &str,
        force: bool,
    ) -> Result<String, DaemonError> {
        let request = Request::new(
            repo_root,
            tool,
            Command::EnqueueSync(EnqueueSyncPayload {
                directory: directory.to_string(),
                force,
            }),
        );
        let response = self.send(request).await?;

        match response.result {
            ResponseResult::Ok { payload } => {
                if let crate::ResponsePayload::EnqueueSync(enqueue) = payload {
                    Ok(enqueue.sync_id)
                } else {
                    Err(DaemonError::Internal("Unexpected response type".into()))
                }
            }
            ResponseResult::Error { error } => Err(DaemonError::Internal(error.message)),
        }
    }

    pub async fn wait_sync(
        &self,
        repo_root: &str,
        tool: &str,
        sync_id: &str,
        timeout_ms: u64,
    ) -> Result<SyncState, DaemonError> {
        let request = Request::new(
            repo_root,
            tool,
            Command::WaitSync(WaitSyncPayload {
                sync_id: sync_id.to_string(),
                timeout_ms,
            }),
        );
        let response = self.send(request).await?;

        match response.result {
            ResponseResult::Ok { payload } => {
                if let crate::ResponsePayload::WaitSync(wait) = payload {
                    Ok(wait.state)
                } else {
                    Err(DaemonError::Internal("Unexpected response type".into()))
                }
            }
            ResponseResult::Error { error } => Err(DaemonError::Internal(error.message)),
        }
    }

    pub async fn sync(
        &self,
        repo_root: &str,
        tool: &str,
        directory: &str,
        wait: bool,
    ) -> Result<SyncState, DaemonError> {
        let sync_id = self.enqueue_sync(repo_root, tool, directory, false).await?;

        if wait {
            self.wait_sync(repo_root, tool, &sync_id, 30_000).await
        } else {
            Ok(SyncState::Queued)
        }
    }

    pub async fn shutdown(&self, reason: &str) -> Result<(), DaemonError> {
        let request = Request::new(
            "",
            "",
            Command::Shutdown(crate::ShutdownPayload {
                reason: reason.to_string(),
            }),
        );
        let _ = self.send(request).await;
        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
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
