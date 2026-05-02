use crate::ports::InferenceEngine;
use anyhow::Result;
use async_trait::async_trait;
use tracing::info;

pub struct LlamaAdapter {
    _model_path: String,
}

impl LlamaAdapter {
    pub fn new(model_path: &str) -> Result<Self> {
        info!("🛠️ [MOCK] Loading model from: {}", model_path);
        Ok(Self { _model_path: model_path.to_string() })
    }
}

#[async_trait]
impl InferenceEngine for LlamaAdapter {
    async fn generate(&self, prompt: &str) -> Result<String> {
        info!("🧠 [MOCK] Generating response for prompt ({} chars)...", prompt.len());
        
        // Simulate a Gemma-4-like technical response
        let response = if prompt.contains("Summarize") {
            "The Rust AI Appliance (V3.5) is a security-first, local-RAG platform. \
            It uses Hexagonal Architecture and a 'Hardened Gatekeeper' to ensure \
            deterministic logic gating and high-efficiency inference."
        } else {
            "Gemma 4 e4b (Effective Series) response: The system is operational and grounded."
        };
        
        Ok(response.to_string())
    }
}
