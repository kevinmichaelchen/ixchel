use anyhow::Result;
pub use helix_embeddings::Embedder;

pub fn create_embedder() -> Result<Embedder> {
    Embedder::new().map_err(|e| anyhow::anyhow!("Failed to create embedder: {e}"))
}

#[cfg(all(test, feature = "embeddings-tests"))]
mod tests {
    use super::*;
    use helix_config::{EmbeddingConfig, load_shared_config};
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_embed_text() {
        if !embeddings_available() {
            return;
        }
        let embedder = create_embedder().unwrap();
        let embedding = embedder.embed("Hello, world!").unwrap();
        assert_eq!(embedding.len(), 384);
    }

    fn configured_model() -> String {
        load_shared_config()
            .map(|config| config.embedding.model)
            .unwrap_or_else(|_| EmbeddingConfig::default().model)
    }

    fn model_cache_paths(model_name: &str) -> Vec<PathBuf> {
        let model_dir = format!("models--{}", model_name.replace('/', "--"));
        let mut roots = Vec::new();

        if let Ok(cache_dir) = env::var("FASTEMBED_CACHE_DIR") {
            roots.push(PathBuf::from(cache_dir));
        } else {
            roots.push(PathBuf::from(".fastembed_cache"));
        }

        if let Ok(hf_home) = env::var("HF_HOME") {
            let hf_home = PathBuf::from(hf_home);
            roots.push(hf_home.clone());
            roots.push(hf_home.join("hub"));
        }

        roots
            .into_iter()
            .map(|root| root.join(&model_dir))
            .collect()
    }

    fn embeddings_available() -> bool {
        let model_name = configured_model();
        let paths = model_cache_paths(&model_name);

        if paths.iter().any(|path| path.exists()) {
            return true;
        }

        let mut message =
            format!("Embeddings test skipped: model cache not found for {model_name}.\n");
        message.push_str("Looked in:\n");
        for path in &paths {
            message.push_str(&format!("  - {}\n", path.display()));
        }
        message.push_str("\nTo run this test:\n");
        message.push_str("  1) Download the model (fastembed caches on first use).\n");
        message.push_str(
            "  2) Re-run with: cargo test -p helix-decisions --features embeddings-tests -- --nocapture\n",
        );
        message
            .push_str("\nYou can control the cache location via FASTEMBED_CACHE_DIR or HF_HOME.\n");
        eprintln!("{message}");

        false
    }
}
