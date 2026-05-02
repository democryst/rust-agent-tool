use serde::Deserialize;
use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

const DEFAULT_RATES: &str = include_str!("../../rates.toml");

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
        let content = if Path::new(path).exists() {
            fs::read_to_string(path)
                .with_context(|| format!("Failed to read rates file at {}", path))?
        } else {
            DEFAULT_RATES.to_string()
        };

        let config: RatesConfig = toml::from_str(&content)
            .context("Failed to parse rates configuration")?;
            
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
