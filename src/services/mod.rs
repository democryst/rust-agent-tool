use crate::domain::{Chunk, Document};
use crate::ports::{Embedder, VectorStore, KnowledgeIngestor};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct IngestionService {
    embedder: Arc<dyn Embedder>,
    vector_store: Arc<dyn VectorStore>,
}

impl IngestionService {
    pub fn new(embedder: Arc<dyn Embedder>, vector_store: Arc<dyn VectorStore>) -> Self {
        Self { embedder, vector_store }
    }
}

#[async_trait]
impl KnowledgeIngestor for IngestionService {
    async fn ingest_document(&self, doc: Document) -> Result<()> {
        // Simple chunking by lines for now
        let chunks: Vec<String> = doc.content.lines()
            .filter(|l| !l.trim().is_empty())
            .map(|s| s.to_string())
            .collect();

        let embeddings = self.embedder.embed_batch(&chunks).await?;

        let domain_chunks = chunks.into_iter().enumerate().map(|(i, content)| {
            Chunk {
                doc_id: doc.id.clone(),
                index: i,
                content,
                embedding: Some(embeddings[i].clone()),
            }
        }).collect();

        self.vector_store.add_chunks(domain_chunks).await?;
        Ok(())
    }
}

pub mod gatekeeper;
pub mod grill;
pub mod orchestrator;
pub mod auditor;
pub mod integrity_gate;
