use crate::domain::{Chunk, VectorSearchResult};
use crate::ports::VectorStore;
use anyhow::Result;
use async_trait::async_trait;
use hnsw_rs::prelude::*;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

pub struct HnswStore {
    index: Arc<RwLock<Hnsw<'static, f32, DistL2>>>,
    chunks: Arc<RwLock<HashMap<usize, Chunk>>>,
}

impl HnswStore {
    pub fn new(max_elements: usize, m: usize, ef_construction: usize) -> Self {
        let index = Hnsw::new(m, max_elements, 16, ef_construction, DistL2 {});
        Self {
            index: Arc::new(RwLock::new(index)),
            chunks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl VectorStore for HnswStore {
    async fn add_chunks(&self, chunks: Vec<Chunk>) -> Result<()> {
        let index = self.index.write().unwrap();
        let mut chunks_map = self.chunks.write().unwrap();

        for chunk in chunks {
            if let Some(ref embedding) = chunk.embedding {
                let id = chunks_map.len();
                index.insert((embedding, id));
                chunks_map.insert(id, chunk);
            }
        }
        Ok(())
    }

    async fn search(&self, query_vector: &[f32], limit: usize) -> Result<Vec<VectorSearchResult>> {
        let index = self.index.read().unwrap();
        let chunks_map = self.chunks.read().unwrap();

        let results = index.search(query_vector, limit, 100);
        let mut search_results = Vec::new();

        for res in results {
            if let Some(chunk) = chunks_map.get(&res.d_id) {
                search_results.push(VectorSearchResult {
                    chunk: chunk.clone(),
                    distance: res.distance,
                });
            }
        }
        Ok(search_results)
    }
}
