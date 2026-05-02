# Comprehensive Instructional Guide: Rust AI Appliance (V3.5)

Welcome to the **Rust AI Appliance**. This guide provides deep-dive instructions on the system's architecture, security protocols, and automated governance.

## 1. Architectural Blueprint: Hexagonal
We use Hexagonal Architecture to separate our business logic from the "infrastructure grime."

- **Domain (`src/domain/`)**: Pure logic, heuristics, and prompt transformation.
- **Ports (`src/ports/mod.rs`)**: Abstract interfaces (Traits) for all external interactions.
- **Adapters (`src/adapters/`)**: Concrete implementations (Vector DB, Model Inference, Crypto, FS Watcher).
- **Services (`src/services/`)**: Orchestration layer that ties everything together.

## 2. The Logic Gate (Hardened Gatekeeper)
The appliance intercepts every query before it reaches the model:
- **Heuristic Parsing**: Prompts are scored for Noun/Verb density and Anchor keyword presence.
- **Efficiency Threshold**: Any prompt scoring below **40/100** is rejected.
- **The Grill**: Rejections return a "Technical Debt" checklist to help you improve your query.
- **SHA-256 Handshake**: Secure string-signing ensures prompts are authorized and vetted.

## 3. Native Inference & RAG
- **Local Gemma-4**: Routine tasks are executed locally on native hardware for maximum privacy and zero latency.
- **Context Injection**: The system automatically retrieves the top-3 most relevant chunks from the `./knowledge` folder to ground the response.
- **Flagship Routing**: Only high-efficiency queries (Score > 85) are routed to Flagship APIs.

## 4. Automated Governance (The Integrity Gate)
This is the final "Guardian" layer that vets the AI's response:
- **Grounding Verification**: Rejects responses that make technical claims without citing source chunks.
- **Evidence-Based Completion**: If the AI says it is "Done," the appliance automatically runs `cargo check`. If the build fails, the response is rejected and replaced with the compiler error.

## 5. Portability & Reliability
The appliance is designed for resilient deployment across different environments:
- **Zero-Config Fallback**: If no `appliance.toml` is found, the binary uses an embedded default configuration.
- **CLI Overrides**: Use `--config` to point to a specific settings file.
- **Graceful Shutdown**: The appliance listens for `Ctrl+C` (SIGINT) to ensure all HNSW indices and logs are flushed to disk before exiting.

## 6. Operations & Costs
- **Port**: The appliance defaults to **`6789`**.
- **Burn Rate Audit**: Real-time USD costing is calculated using `rates.toml`.
- **Management**: Use the `Makefile` for setup, building, and downloading models.

## 6. How to Ground the AI
Simply drop any `.txt` or `.md` file into the `knowledge/` folder. The `FileWatcher` will automatically detect the change, generate embeddings, and index the content in the HNSW store.

---

> "The Rust logic gate is the law; the LLM is merely the worker."
