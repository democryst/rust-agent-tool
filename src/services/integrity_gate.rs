use crate::domain::governance::{IntegrityScorer, IntegrityResult};
use crate::ports::{IntegrityVetter, ActionVerifier};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

pub struct IntegrityGateService {
    verifier: Arc<dyn ActionVerifier>,
}

impl IntegrityGateService {
    pub fn new(verifier: Arc<dyn ActionVerifier>) -> Self {
        Self { verifier }
    }
}

#[async_trait]
impl IntegrityVetter for IntegrityGateService {
    async fn vet_output(&self, output: &str) -> Result<IntegrityResult> {
        // 1. Grounding Verification
        if !IntegrityScorer::verify_grounding(output) {
            return Ok(IntegrityResult::GroundedFailure(
                "Integrity Violation: No citations found. Technical claims must be grounded.".to_string()
            ));
        }

        // 2. Evidence-Based Completion Check
        if IntegrityScorer::detect_completion_intent(output) {
            match self.verifier.verify_system_state().await {
                Ok(true) => Ok(IntegrityResult::Approved),
                Ok(false) => unreachable!(), // Handled by Err
                Err(e) => Ok(IntegrityResult::VerificationFailure(
                    format!("Integrity Violation: Completion claimed but system check failed:\n{}", e)
                )),
            }
        } else {
            Ok(IntegrityResult::Approved)
        }
    }
}
