# Rust SignalForge Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `rust-signalforge`, a recruiter-facing Rust API that combines engineering signal scoring with CargoPulse-style service health monitoring.

**Architecture:** The app is a small Axum HTTP service with pure domain modules for scoring and pulse evaluation. Routes call pure functions so tests can cover business behavior and HTTP behavior separately.

**Tech Stack:** Rust, Tokio, Axum, Serde, Tower, tracing, integration tests with `tower::ServiceExt`.

## Global Constraints

- Repository location: `D:\Rust\rust-signalforge`.
- Project name must start with `rust-` and have a creative name.
- No secrets in repository files.
- Preserve recruiter-friendly docs explaining Rust, Node.js, and NestJS differences.
- Verify with `cargo fmt`, `cargo test`, and `cargo build` when Rust toolchain is available.

---

### Task 1: Project Skeleton

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`
- Create: `src/lib.rs`
- Create: `src/main.rs`

**Interfaces:**
- Produces: `rust_signalforge::app() -> axum::Router`

- [x] Create the Axum application entrypoint and library boundary.
- [x] Keep route construction in the library so integration tests can call it.

### Task 2: SignalForge Scoring

**Files:**
- Create: `src/models.rs`
- Create: `src/scoring.rs`
- Test: `tests/api.rs`

**Interfaces:**
- Produces: `evaluate_signal(request: SignalRequest) -> SignalReport`

- [x] Add evidence scoring for impact, reliability, performance, ownership, documentation, and learning.
- [x] Expose `/api/signals` and `/api/signals/evaluate`.

### Task 3: CargoPulse Monitor

**Files:**
- Create: `src/pulse.rs`
- Modify: `src/routes.rs`
- Test: `tests/api.rs`

**Interfaces:**
- Produces: `evaluate_pulse(request: PulseRequest) -> PulseReport`

- [x] Add typed health check aggregation with critical failure handling.
- [x] Expose `/health`, `/api/pulse`, and `/api/pulse/evaluate`.

### Task 4: Recruiter Documentation

**Files:**
- Create: `README.md`
- Create: `docs/PRESENTING.md`

**Interfaces:**
- Produces: clear run steps, endpoint examples, interview talking points, and Rust vs Node.js/NestJS notes.

- [x] Explain what the app proves.
- [x] Include presentation script and technical comparison.

### Task 5: Verification and Publish

**Files:**
- Modify only if verification reveals issues.

**Interfaces:**
- Produces: pushed GitHub repository.

- [ ] Run `cargo fmt`, `cargo test`, and `cargo build`.
- [ ] Commit the complete project.
- [ ] Create or use GitHub repo `rust-signalforge`.
- [ ] Push the branch.
