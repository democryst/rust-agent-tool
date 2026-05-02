use crate::ports::Embedder;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use std::sync::Mutex;

pub struct FastEmbedder {
    model: Mutex<TextEmbedding>,
}

impl FastEmbedder {
    pub fn new() -> Result<Self> {
        let mut options = InitOptions::default();
        options.model_name = EmbeddingModel::BGESmallENV15;
        options.show_download_progress = true;
        
        let model = TextEmbedding::try_new(options)?;
        Ok(Self { model: Mutex::new(model) })
    }
}

#[async_trait]
impl Embedder for FastEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let mut model = self.model.lock().map_err(|_| anyhow!("Failed to lock model"))?;
        let embeddings = model.embed(vec![text], None)?;
        embeddings.into_iter().next().ok_or_else(|| anyhow!("Failed to generate embedding"))
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let mut model = self.model.lock().map_err(|_| anyhow!("Failed to lock model"))?;
        let embeddings = model.embed(texts.to_vec(), None)?;
        Ok(embeddings)
    }
}
