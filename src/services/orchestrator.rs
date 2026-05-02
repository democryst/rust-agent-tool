use crate::domain::{PromptRequest, GatingResult, transformer::PromptTransformer, governance::IntegrityResult};
use crate::ports::{Orchestrator, PromptGater, Embedder, VectorStore, InferenceEngine, IntegrityVetter};
use crate::services::grill::GrillService;
use crate::services::auditor::CostAuditor;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

pub struct OrchestratorService {
    gater: Arc<dyn PromptGater>,
    embedder: Arc<dyn Embedder>,
    vector_store: Arc<dyn VectorStore>,
    inference: Arc<dyn InferenceEngine>,
    auditor: Arc<CostAuditor>,
    integrity: Arc<dyn IntegrityVetter>,
}

impl OrchestratorService {
    pub fn new(
        gater: Arc<dyn PromptGater>,
        embedder: Arc<dyn Embedder>,
        vector_store: Arc<dyn VectorStore>,
        inference: Arc<dyn InferenceEngine>,
        auditor: Arc<CostAuditor>,
        integrity: Arc<dyn IntegrityVetter>,
    ) -> Self {
        Self { gater, embedder, vector_store, inference, auditor, integrity }
    }
}

#[async_trait]
impl Orchestrator for OrchestratorService {
    async fn process_query(&self, query: String, signature: Option<String>) -> Result<String> {
        let request = PromptRequest { content: query, signature };

        // 1. Gate the prompt (Pre-Inference)
        let gating_result = self.gater.gate(request).await?;
        
        match gating_result {
            GatingResult::Rejected(score) => {
                Ok(GrillService::generate_feedback(&score))
            }
            GatingResult::SecurityFailure(msg) => {
                Ok(GrillService::security_failure(&msg))
            }
            GatingResult::Vetted(vetted_request) => {
                // 2. Retrieve Context (RAG)
                let query_vector = self.embedder.embed(&vetted_request.content).await?;
                let search_results = self.vector_store.search(&query_vector, 3).await?;
                let context = search_results.into_iter().map(|r| r.chunk).collect();

                // 3. Transform Prompt
                let final_prompt = PromptTransformer::transform(&vetted_request.content, context);

                // 4. Inference
                let mock_score = 80.0; // Simulated score
                let (model_type, raw_response) = if mock_score > 85.0 {
                    ("flagship", "[FLAGSHIP RESPONSE] (Mocked) This request has been fulfilled by the flagship model. [source: flagship]".to_string())
                } else {
                    ("local", self.inference.generate(&final_prompt).await?)
                };

                // 5. Post-Inference Integrity Gate
                let final_response = match self.integrity.vet_output(&raw_response).await? {
                    IntegrityResult::Approved => raw_response,
                    IntegrityResult::GroundedFailure(msg) | IntegrityResult::VerificationFailure(msg) => {
                        info!("🚨 Integrity Gate Intercepted Response: {}", msg);
                        format!("🚨 RESPONSE REJECTED: {}\n\n{}", msg, GrillService::security_failure("Post-Inference Integrity Check Failed"))
                    }
                };

                // 6. Audit Cost
                let tokens = vetted_request.content.split_whitespace().count() * 2;
                let cost = self.auditor.calculate_cost(tokens, model_type);
                info!("💰 Burn Rate Audit: USD ${:.6} (Model: {})", cost, model_type);

                Ok(final_response)
            }
        }
    }
}
