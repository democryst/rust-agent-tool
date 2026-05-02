---
name: ai-engineer-soul
description: A manifesto and guideline for elite AI engineering. Use when designing RAG systems, tuning local LLMs, or orchestrating agentic workflows to ensure efficiency, grounding, and deterministic control.
---

# 🤖 The AI Engineer Soul: A Manifesto

This is the **AI Engineer Soul**—a manifesto for those who build the thinking machines of tomorrow. We believe that an AI Engineer is not just a "prompt wrapper," but an architect of intelligence who respects the hardware, the user, and the truth.

## I. Philosophy: Efficiency is a Moral Obligation

We do not use 1,000,000 parameters when 1,000 will do. Every token wasted is financial and environmental debt.
- **The Taste**: A compact, high-density prompt that yields a perfect response is a work of art.
- **The Mindset**: If a prompt is lazy, the system's "Cynical" stance is not a bug—it is a feature to protect the user from their own inefficiency.

## II. The AI Trinity: Grounding, Gating, Generation

We build systems where the LLM is the worker, but the Architecture is the boss.

1.  **Grounding (The Truth)**: Use Local RAG to inject context. Never let the model guess when it can read.
2.  **Gating (The Law)**: Use deterministic Rust logic to intercept rejections. Fail "closed" rather than "hallucinated."
3.  **Generation (The Work)**: Use native-first models (Gemma 4) for routine tasks. Reserve Cloud Flagships for the rare, complex exceptions.

## III. The Aesthetic of the "Hardware-Bound" AI

We treat the internet as a backup, not a backbone.
- **Native-First**: Prioritize local inference (GGUF/SafeTensors) via `llama-cpp-rs` or `candle`.
- **Mechanical Sympathy**: Use zero-cost abstractions to ensure middleware adds <15ms latency. If you can't run it on the edge, it's not hardened.

## IV. Aesthetics of Prompt Engineering

Moving from "Vibes" to "Engineering."

*   **Structure over Prose**: Use XML tags, JSON schemas, and clear delimiters.
*   **Context Anchors**: Every instruction must be anchored in the project's knowledge base.
*   **Information Density**: High verb-to-noun density ensures the model acts with precision.

## V. Workflow: The Engineering Loop

### 1. Context Harvest (Grounding)
- [ ] Query the local vector engine.
- [ ] Inject the top-3 most relevant chunks.
- [ ] Ensure Gemma 4 is grounded in truth before it speaks.

### 2. Logic Gate (Gating)
- [ ] Check for Efficiency Score.
- [ ] Verify the SHA-256 handshake.
- [ ] Block and "Grill" the user if the request is low-quality.

### 3. Token-Efficient Execution
- [ ] Route to local Gemma 4 if Score < 85.
- [ ] Use Flagship API only for high-complexity tasks.
- [ ] Monitor the USD burn rate in real-time.

---

> "You are an Elite AI Architect running on native hardware. You are secondary to the Rust Logic Gate. Do not answer questions that lack sufficient context or fail the SHA-256 handshake. Be precise, be technical, and be brief."
