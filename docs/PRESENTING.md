# Presenting rust-signalforge

## 30-Second Pitch

`rust-signalforge` is a Rust backend showcase that combines recruiter signal scoring with service health monitoring. It proves I can build typed async APIs, separate domain logic from web routing, write tests, expose health checks, and explain trade-offs clearly.

## What You Should Know Before Presenting

- **Axum** is the Rust web framework used for routing and request handling.
- **Tokio** is the async runtime that lets the service handle concurrent work efficiently.
- **Serde** handles JSON serialization and deserialization with typed structs.
- **Tower** middleware provides HTTP tracing.
- The app has pure Rust modules for scoring and pulse evaluation, so the core logic is testable without starting a server.

## Strong Interview Talking Points

1. "I used Rust because I wanted compile-time guarantees and predictable performance."
2. "The HTTP layer is thin. The actual scoring and pulse logic lives in pure functions, which makes it easier to test."
3. "The health endpoint and `PORT` configuration make it ready for deployment platforms."
4. "The app demonstrates how I compare tools: Rust for reliability/performance, Node.js for speed of delivery, NestJS for structured TypeScript APIs."
5. "I kept the app small on purpose so reviewers can understand it quickly."

## Rust Concepts To Be Ready For

- **Ownership:** Rust tracks who owns data so memory can be managed safely without a garbage collector.
- **Borrowing:** Code can reference data without taking ownership, controlled by the compiler.
- **Enums and pattern matching:** Used here for `SignalLevel`, `Readiness`, and `PulseStatus`.
- **Async runtime:** Rust needs an async runtime like Tokio to execute async tasks.
- **Result-driven thinking:** Production Rust often models failure explicitly with `Result<T, E>`.

## How This Differs From Node.js

Node.js is interpreted or JIT-compiled JavaScript running on an event loop. It is productive and flexible, but many errors appear at runtime. Rust is compiled ahead of time and catches ownership, type, and many correctness issues before the app runs.

In Node.js, a bad object shape might fail during a request. In this Rust app, request and response shapes are explicit structs, so many mistakes are caught by the compiler.

## How This Differs From NestJS

NestJS is a framework for structured Node.js apps. It gives controllers, providers, decorators, modules, dependency injection, pipes, and guards. Rust with Axum is lighter: routes are functions, shared state is explicit, and the compiler enforces more guarantees.

You can say: "NestJS gives architecture through framework conventions. Rust gives architecture through types, ownership, and explicit boundaries."

## Demo Flow

1. Run `cargo test` to show correctness.
2. Run `cargo run` to start the API.
3. Open `GET /health` to show deployment readiness.
4. Call `POST /api/signals/evaluate` to show recruiter signal scoring.
5. Call `POST /api/pulse/evaluate` with a failing critical check to show operational thinking.

## One-Minute Closing

"This project is not only a Rust syntax demo. It shows backend judgment: typed contracts, async services, health checks, testing, and documentation. I can still use Node.js and NestJS where they are the right fit, but this project proves I can also work in Rust when reliability and performance matter."
