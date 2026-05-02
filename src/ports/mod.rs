use crate::domain::{Chunk, Document, VectorSearchResult, PromptRequest, GatingResult, governance::IntegrityResult};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}

#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn add_chunks(&self, chunks: Vec<Chunk>) -> Result<()>;
    async fn search(&self, query_vector: &[f32], limit: usize) -> Result<Vec<VectorSearchResult>>;
}

#[async_trait]
pub trait KnowledgeIngestor: Send + Sync {
    async fn ingest_document(&self, doc: Document) -> Result<()>;
}

#[async_trait]
pub trait PromptGater: Send + Sync {
    async fn gate(&self, request: PromptRequest) -> Result<GatingResult>;
}

pub trait CryptoService: Send + Sync {
    fn hash_sha256(&self, data: &str) -> String;
    fn verify_signature(&self, data: &str, signature: &str) -> bool;
}

#[async_trait]
pub trait InferenceEngine: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
}

#[async_trait]
pub trait Orchestrator: Send + Sync {
    async fn process_query(&self, query: String, signature: Option<String>) -> Result<String>;
}

#[async_trait]
pub trait IntegrityVetter: Send + Sync {
    async fn vet_output(&self, output: &str) -> Result<IntegrityResult>;
}

#[async_trait]
pub trait ActionVerifier: Send + Sync {
    async fn verify_system_state(&self) -> Result<bool, String>;
}
