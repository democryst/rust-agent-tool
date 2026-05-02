---
name: rust-soul
description: A manifesto and guideline for idiomatic, high-performance, and safe Rust development. Use when building or refactoring Rust code to ensure "mechanical sympathy," type-safety, and adherence to the "Rust Soul."
---

# 🦀 The Rust Soul: A Manifesto

This is the **Rust Soul**—a manifesto of the developer who values the machine, the user, and the future of the code equally. This is the "Tao of Rust" that bridges systems, web, and WASM.

## I. Philosophy: The Compiler is a Peer

In the Rust soul, we do not "fight" the borrow checker. We recognize that its errors are not criticisms, but **proofs**.

*   **The Taste:** If the code compiles, it works. We move the effort of debugging from "Production at 3 AM" to "Development at 2 PM."
*   **The Mindset:** We view `panic!` as a failure of imagination. We prefer to handle every `None` and `Err` with grace.

## II. The Trinity of Control

Whether we are in the kernel, the server, or the browser, we adhere to three laws:

1.  **Memory is Sacred:** We do not leak, we do not double-free, and we do not permit data races.
2.  **Abstractions Must be Free:** We use high-level traits and iterators, knowing they compile down to the same assembly as the low-level grime.
3.  **Fearless Concurrency:** We share data across threads not with "hope," but with `Arc`, `Mutex`, and `Send`/`Sync` traits that guarantee safety.

## III. The Aesthetic of the "Type-Safe Fortress"

A Rustacean’s "taste" is defined by making **illegal states unrepresentable.**

*   **Bad Taste:** Using a `String` to hold an IP address.
*   **Rust Soul:** Creating an enum `IpAddr { V4(Ipv4Addr), V6(Ipv6Addr) }`.
*   **The Goal:** If a state shouldn't exist, the code shouldn't compile.

## IV. The Three Realms

### ⚙️ Systems: The Foundation
*   **The Soul:** We write code that respects the silicon. We manage resources via **RAII**. When a variable goes out of scope, the file closes, the memory frees, and the lock releases. No garbage collector needed; just deterministic elegance.

### 🌐 Web: The Scale
*   **The Soul:** We build backends that don't crash under load. We use `Serde` to transform data with zero-copy efficiency. Our APIs are contracts signed in the blood of the type system.

### 🕸️ WASM: The Boundary
*   **The Soul:** We bring the power of the desktop to the web. We don't replace JavaScript; we augment it. We write the performance-critical core in Rust and export a clean, safe interface to the browser.

## V. Workflow: Applying the Soul

When you are writing Rust code, follow this loop:

### 1. Type-First Design
Before writing logic, define your types. 
- [ ] Use `enum` for state machines and variants.
- [ ] Use `struct` for data containers.
- [ ] Avoid "Primitive Obsession" (e.g., don't use `i32` for a `UserAge` if you can use a Newtype).

### 2. The Clippy Check
Run `cargo clippy` early and often. It is the voice of the soul's experience.

### 3. Handle the Edge
Every `unwrap()` is a debt. 
- [ ] Replace `unwrap()` with `expect("contextual message")` if failure is impossible.
- [ ] Use `?` for error propagation in fallible paths.

### 4. Zero-Cost Verification
Use iterators over manual loops where possible. Trust the optimizer.

---

## Tooling (The Cargo Cult)

We do not struggle with build systems.
*   We worship **Cargo** for it is the best-in-class package manager.
*   We listen to **Clippy**, for it knows the idiomatic path.
*   We document our code with **Doc-Tests**, ensuring our promises never lie to the user.

> "To write Rust is to accept that the cost of correctness is paid upfront, in full, so that the dividends of stability may be collected forever."

**Are you ready to commit your first main.rs and let the borrow checker refine your soul?**