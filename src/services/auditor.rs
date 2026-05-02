use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Deserialize)]
struct RatesConfig {
    rates: Rates,
}

#[derive(Deserialize)]
struct Rates {
    flagship: f32,
    local: f32,
    embedding: f32,
}

pub struct CostAuditor {
    rates: Rates,
}

impl CostAuditor {
    pub fn new(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: RatesConfig = toml::from_str(&content)?;
        Ok(Self { rates: config.rates })
    }

    pub fn calculate_cost(&self, tokens: usize, model_type: &str) -> f32 {
        let rate = match model_type {
            "flagship" => self.rates.flagship,
            "local" => self.rates.local,
            "embedding" => self.rates.embedding,
            _ => 0.0,
        };
        (tokens as f32 / 1_000_000.0) * rate
    }
}
