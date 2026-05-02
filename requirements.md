🛠️ Project: The Rust AI Appliance (V3.5)

Suggestive Griller | Local RAG | Gemma 4 Native Inference | Automated Governance

1. Requirements Specification

🛡️ The "Hardened" Gatekeeper (Middleware)

- Deterministic Blocking: Intercept prompts with Efficiency Score < 40/100. [DONE]
- Suggestive Feedback: "Technical Debt" checklist via the Grill. [DONE]
- Burn Rate Audit: Real-time costing via rates.toml ($5.00 Flagship / $0.05 Local). [DONE]
- Loop Protection: SHA-256 handshake verification. [DONE]

🧠 Native Inference Engine (Gemma 4 e4b)

- Direct Execution: Gemma 4 e4b (Mocked for current environment). [DONE]
- Zero-API Routing: Score > 85 routes to Flagship; Score 40-85 stays Local. [DONE]
- Hardware Acceleration: Configured for Metal/CUDA compatibility. [DONE]

📂 Knowledge-Base (Local RAG)

- Automated Ingestion: Notify-based watcher on /knowledge. [DONE]
- Vector Engine: Local BGE-Small-EN-v1.5 embeddings. [DONE]
- Context Injection: Top-3 chunk retrieval and injection. [DONE]

🛡️ Automated Governance (The Integrity Gate)

- Grounding Verification: Post-inference citation check. [DONE]
- Evidence-Based Completion: Automated `cargo check` on completion claims. [DONE]

2. Development Plan (Roadmap)

Phase 1: The Ingestion & Memory Layer
[x] Filesystem Watcher: Implement notify crate.
[x] Chunking & Embedding: recursive splitter + fastembed.
[x] Vector Storage: Local HNSW index.

Phase 2: The Logic Gate & Heuristics
[x] Heuristic Parser: Noun/Verb density + Anchor scanner.
[x] The "Grill" UI: Suggestive feedback generator.
[x] Security: SHA-256 string-signing.

Phase 3: Native Inference Integration
[x] Model Loading: llama-cpp-rs integration (Architected).
[x] Prompt Transformer: Context + System merge.
[x] Orchestrator: Match logic (Reject / Local / Flagship).

Phase 4: Hardening & Governance
[x] Burn Rate Auditor: costing logic and rates.toml.
[x] Integrity Gate: Post-inference citation and system verification.
[x] Build System: Makefile and standalone CLAUDE.md guide.

3. Developer Mindset & "Taste"

I. Efficiency is a Moral Obligation
II. Native-First, Cloud-Last
III. Determinism over Hallucination
IV. Mechanical Sympathy

4. Final System Prompt

"You are an Elite AI Architect running on native hardware. You are grounded by the /knowledge vector store. You are secondary to the Rust Logic Gate. Do not answer questions that lack sufficient context or fail the SHA-256 handshake. Be precise, be technical, and be brief."
