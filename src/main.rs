use rust_agent_tool::adapters::fastembed_adapter::FastEmbedder;
use rust_agent_tool::adapters::hnsw_adapter::HnswStore;
use rust_agent_tool::adapters::notify_adapter::FileWatcher;
use rust_agent_tool::adapters::crypto_adapter::Sha2CryptoAdapter;
use rust_agent_tool::adapters::llama_adapter::LlamaAdapter;
use rust_agent_tool::adapters::system_verifier::CargoVerifier;
use rust_agent_tool::services::IngestionService;
use rust_agent_tool::services::gatekeeper::GatekeeperService;
use rust_agent_tool::services::orchestrator::OrchestratorService;
use rust_agent_tool::services::auditor::CostAuditor;
use rust_agent_tool::services::integrity_gate::IntegrityGateService;
use rust_agent_tool::ports::Orchestrator;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, warn};
use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use clap::Parser;
use tokio::signal;

const DEFAULT_CONFIG: &str = include_str!("../appliance.toml");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file (optional, will use embedded default if missing)
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ApplianceConfig {
    knowledge_path: String,
    models_path: String,
    rates_path: String,
    server: ServerConfig,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct ServerConfig {
    port: u16,
    host: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct QueryRequest {
    query: String,
    signature: Option<String>,
}

#[derive(Serialize)]
#[allow(dead_code)]
struct QueryResponse {
    response: String,
}

#[allow(dead_code)]
struct AppState {
    orchestrator: Arc<dyn Orchestrator>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("🚀 Initializing Rust AI Appliance (V3.5)...");

    // 0. Resolve Configuration
    let config: ApplianceConfig = if let Some(path) = args.config {
        info!("📖 Loading external config from: {:?}", path);
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read external config at {:?}", path))?;
        toml::from_str(&content).context("Failed to parse external config")?
    } else if Path::new("appliance.toml").exists() {
        info!("📖 Loading local appliance.toml...");
        let content = fs::read_to_string("appliance.toml")
            .context("Failed to read local appliance.toml")?;
        toml::from_str(&content).context("Failed to parse local appliance.toml")?
    } else {
        info!("📦 Using embedded default configuration.");
        toml::from_str(DEFAULT_CONFIG).context("Failed to parse embedded config")?
    };

    // 1. Setup Infrastructure
    let embedder = Arc::new(FastEmbedder::new()
        .context("Failed to initialize FastEmbedder")?);
    let vector_store = Arc::new(HnswStore::new(10000, 16, 200));
    let crypto = Arc::new(Sha2CryptoAdapter);
    let verifier = Arc::new(CargoVerifier);
    
    let model_file = format!("{}/gemma-4-e4b.gguf", config.models_path);
    let llama = Arc::new(LlamaAdapter::new(&model_file)
        .with_context(|| format!("Failed to load model file at {}. Did you run 'make download-model'?", model_file))?);
    let auditor = Arc::new(CostAuditor::new(&config.rates_path)
        .context("Failed to initialize CostAuditor")?);

    // 2. Setup Services
    let anchors = vec!["Rust".to_string(), "AI".to_string(), "Appliance".to_string(), "Gatekeeper".to_string()];
    let gater = Arc::new(GatekeeperService::new(crypto.clone(), anchors));
    let integrity = Arc::new(IntegrityGateService::new(verifier.clone()));
    let _ingestor = Arc::new(IngestionService::new(embedder.clone(), vector_store.clone()));
    
    let orchestrator = Arc::new(OrchestratorService::new(
        gater.clone(),
        embedder.clone(),
        vector_store.clone(),
        llama.clone(),
        auditor.clone(),
        integrity.clone(),
    ));

    // 3. Setup Ingestion Watcher
    let (tx, mut _rx) = mpsc::channel(100);
    tokio::fs::create_dir_all(&config.knowledge_path).await?;
    let _watcher = FileWatcher::new(&config.knowledge_path, tx)?;
    info!("📂 Watching knowledge directory: {}", config.knowledge_path);

    // 4. Sample Interaction (Simulated)
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        let query = "Explain how the gatekeeper works.";
        match orchestrator.process_query(query.to_string(), None).await {
            Ok(response) => info!("🤖 Response:\n{}", response),
            Err(e) => warn!("❌ Error: {:?}", e),
        }
    });

    // 5. Graceful Shutdown
    info!("🛎️  Press Ctrl+C to terminate the appliance.");
    
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("🛑 Termination signal received. Shutting down gracefully...");
        }
        Err(err) => {
            warn!("❌ Unable to listen for shutdown signal: {}", err);
        }
    }

    Ok(())
}
