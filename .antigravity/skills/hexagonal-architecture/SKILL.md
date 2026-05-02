---
name: hexagonal-architecture
description: Guidelines for implementing Ports and Adapters (Hexagonal Architecture). Use when designing new services, refactoring for testability, or isolating business logic from external dependencies like databases, APIs, and file systems.
---

# ⬢ Hexagonal Architecture (Ports & Adapters)

Hexagonal Architecture focuses on keeping the **Domain Logic** at the center of the application, completely isolated from external concerns. External systems (databases, web frameworks, external APIs) are treated as **Adapters** that plug into **Ports** (interfaces) defined by the core.

## 1. Core Philosophy: Dependency Inversion

In a Hexagonal system:
- **Domain** depends on nothing.
- **Application Services** depend on the Domain and Ports.
- **Infrastructure/Adapters** depend on Ports.

Dependencies always point **inwards** toward the business logic.

## 2. The Three Layers

### 🏛️ The Domain (Center)
- **Entities & Value Objects**: Pure business logic and state.
- **Domain Services**: Logic that doesn't fit into a single entity.
- **No IO**: This layer must not perform database calls, HTTP requests, or logging.

### 🔌 Ports (Interfaces)
- Defined by the Application layer.
- **Input Ports**: Public API of the application (e.g., Use Case traits).
- **Output Ports**: Requirements the application has from the outside world (e.g., `Repository` traits, `EmailSender` traits).

### 🛠️ Adapters (Implementation)
- Implementations of the Ports.
- **Primary Adapters**: Drive the application (CLI, REST Controllers, Event Consumers).
- **Secondary Adapters**: Driven by the application (Postgres implementation, S3 storage, SendGrid client).

## 3. Workflow: Adding a Feature

Follow these steps to implement a new vertical slice:

### Step 1: Define the Domain Model
- Create entities or value objects in the `domain` module.
- Ensure they are "Type-Safe" (see [rust-soul](../rust-soul/SKILL.md)).

### Step 2: Define Output Ports
- Create traits for required infrastructure (e.g., `trait UserRepository`).

### Step 3: Implement Use Case (Application Service)
- Create a struct that orchestrates domain objects.
- It should use the Output Port traits via dependency injection (generics or boxed traits).

### Step 4: Implement Adapters
- Create concrete implementations of the Output Ports in the `infrastructure` or `adapters` module.
- Create Primary Adapters (e.g., an Actix-web handler) that call the Use Case.

## 4. Anti-Patterns to Avoid

- [ ] **Leaky Abstractions**: Do not use database-specific types (e.g., `diesel` or `sqlx` models) in your Domain or Ports.
- [ ] **Infrastructure in Domain**: Do not put connection pools or client configurations inside your core logic.
- [ ] **God Use Cases**: Keep your Application Services focused on a single orchestration task.

---

> "The goal of Hexagonal Architecture is to allow an application to equally be driven by users, programs, automated tests, or batch scripts, and to be developed and tested in isolation from its eventual run-time devices and databases."
