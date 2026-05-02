---
name: security
description: Guidelines for implementing the Hardened Gatekeeper and system security. Use when working on authentication, prompt vetting, SHA-256 handshakes, or any security-critical middleware.
---

# 🛡️ Security: The Hardened Gatekeeper

In the Rust AI Appliance, security is not an afterthought—it is a deterministic logic gate that sits between the user and the LLM. We follow a **fail-closed** philosophy to protect resources and ensure truth.

## 1. Core Philosophy: Fail-Closed

- **The Law of the Gate**: If the system is unsure, it must reject the request.
- **Deterministic First**: Security checks must be implemented in pure Rust logic, not delegated to the LLM.
- **Zero Trust**: Every prompt is treated as potentially malicious or inefficient until proven otherwise by the SHA-256 handshake.

## 2. The SHA-256 Handshake

All "vetted" prompts must carry a signature to prevent recursive tool-calls and unauthorized access.

### Verification Workflow:
- [ ] **Extract Signature**: Find the `<vetted_sha256="..." />` tag in the prompt.
- [ ] **Hash Content**: Compute the SHA-256 hash of the prompt body (excluding the tag).
- [ ] **Compare**: If the computed hash does not match the signature, the request **must fail**.
- [ ] **Fail Closed**: Return a `SecurityViolation` error; do not pass the prompt to the model.

## 3. Deterministic Blocking & Vetting

Before reaching the model, prompts must pass the **Efficiency & Safety Grill**.

### Vetting Criteria:
- **Efficiency Score**: Reject prompts with an Efficiency Score < 40/100.
- **Noun/Verb Density**: Check for low information density.
- **Anchor Keywords**: Ensure the prompt contains required context anchors.

### Rejection Protocol:
When a prompt is blocked, return a specific **"Technical Debt" checklist**:
- Missing Anchors
- Low Verb-to-Noun density
- Security Handshake Failure

## 4. Secure Development Checklist

- [ ] **Dependency Audit**: Run `cargo audit` to check for known vulnerabilities in crates.
- [ ] **No Secrets in Code**: Ensure API keys or USD rates are never hardcoded (use `Env` or `rates.toml`).
- [ ] **Memory Safety**: Adhere to the [Rust Soul](../rust-soul/SKILL.md) to prevent memory-based attacks.
- [ ] **Sanitize Inputs**: Always sanitize data before it enters the local RAG vector engine.

---

> "The Rust logic gate is the law; the LLM is merely the worker. We prefer a 'Technical Debt' rejection over a 'Maybe' answer."
