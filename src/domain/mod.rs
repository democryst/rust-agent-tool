use serde::{Deserialize, Serialize};

pub mod heuristics;
pub mod transformer;
pub mod governance;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub doc_id: String,
    pub index: usize,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
}

pub struct VectorSearchResult {
    pub chunk: Chunk,
    pub distance: f32,
}

#[derive(Debug, Clone)]
pub struct PromptRequest {
    pub content: String,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VettingScore {
    pub noun_density: f32,
    pub verb_density: f32,
    pub anchor_status: bool,
    pub total_efficiency: f32,
}

pub enum GatingResult {
    Vetted(PromptRequest),
    Rejected(VettingScore),
    SecurityFailure(String),
}
