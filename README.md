# 🛠️ The Rust AI Appliance (V3.5)

**Suggestive Griller | Local RAG | Gemma 4 Native Inference | Automated Governance**

The Rust AI Appliance is a high-performance, security-first AI platform. It acts as a "Hardened Gatekeeper," ensuring all AI interactions are grounded, secured by SHA-256 handshakes, and verified by automated technical checks.

## 📥 Installation Guide

### 1. Prerequisites
- **Rust**: Install via [rustup.rs](https://rustup.rs/).
- **C++ Toolchain**: Required for model inference (e.g., Xcode on Mac, Build Tools on Windows).
- **Git**: To clone the repository.

### 2. Quick Setup
Clone the repository and run the setup helper:
```bash
make setup
```
This creates the necessary `./knowledge` and `./models` directories and initializes your `appliance.toml`.

### 3. Download the Brain (Gemma 4 e4b)
Download the recommended 4-bit quantized model:
```bash
make download-model
```

### 4. Build and Run
Build the release binary and start the appliance:
```bash
make build
make run
```
The appliance will now be listening at `http://127.0.0.1:6789`.

---

## ⚙️ Configuration (`appliance.toml`)

Customize your appliance paths and server settings:
```toml
knowledge_path = "./knowledge"
models_path = "./models"
rates_path = "rates.toml"

[server]
port = 6789
host = "127.0.0.1"
```

---

## 🤖 How to Use

Once the appliance is running, you can interact with it via its REST API. 

### Integration
Copy the **[CLAUDE.md](./CLAUDE.md)** file from this repository into any project where you want to enable grounded AI development. It contains the rules and protocol for calling this appliance.

### Example Query
```bash
curl -X POST http://127.0.0.1:6789/query \
     -H "Content-Type: application/json" \
     -d '{"query": "Summarize the project goals"}'
```

---

## 🛡️ Automated Governance
The appliance enforces a two-way gate:
1.  **Logic Gate (Pre-Inference)**: Rejects lazy or unanchored prompts.
2.  **Integrity Gate (Post-Inference)**: Rejects hallucinated answers and automatically runs `cargo check` to verify successful implementation claims.

---

> "Efficiency is a Moral Obligation. The Rust logic gate is the law; the LLM is merely the worker."
