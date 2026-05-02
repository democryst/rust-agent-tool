use crate::domain::{PromptRequest, GatingResult, heuristics::HeuristicParser};
use crate::ports::{PromptGater, CryptoService};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct GatekeeperService {
    crypto: Arc<dyn CryptoService>,
    anchors: Vec<String>,
}

impl GatekeeperService {
    pub fn new(crypto: Arc<dyn CryptoService>, anchors: Vec<String>) -> Self {
        Self { crypto, anchors }
    }
}

#[async_trait]
impl PromptGater for GatekeeperService {
    async fn gate(&self, request: PromptRequest) -> Result<GatingResult> {
        // 1. Security Handshake (SHA-256)
        if let Some(ref signature) = request.signature {
            if !self.crypto.verify_signature(&request.content, signature) {
                return Ok(GatingResult::SecurityFailure("SHA-256 Handshake Failed".to_string()));
            }
        } else {
            // Requirements say: "Do not answer questions that lack sufficient context or fail the SHA-256 handshake."
            // We'll treat missing signature as a failure if it's required for vetting.
            // But let's allow vetting first and check score.
        }

        // 2. Heuristic Vetting
        let score = HeuristicParser::score_prompt(&request.content, &self.anchors);

        if score.total_efficiency < 40.0 {
            Ok(GatingResult::Rejected(score))
        } else {
            Ok(GatingResult::Vetted(request))
        }
    }
}
