# rust-signalforge

`rust-signalforge` is a recruiter-facing Rust showcase API. It combines two ideas:

- **SignalForge:** turns project evidence into a recruiter-readable engineering signal report.
- **CargoPulse:** evaluates service health checks, latency, and critical failures.

The app is intentionally small, but it is built like a real backend service: typed request/response models, async routes, pure domain logic, integration tests, health checks, structured logging, and deployment-friendly configuration.

## Why This Exists

Recruiters and engineering managers often need fast proof that a candidate can build reliable backend software. This project gives them a concrete Rust example that demonstrates:

- Async API development with Axum and Tokio.
- Strong typing with Serde request/response models.
- Business logic separated from HTTP routing.
- Health and readiness thinking.
- Tests around both domain behavior and HTTP behavior.
- Clear documentation and presentation notes.

## Quick Start

```powershell
cargo test
cargo run
```

The service starts on port `8080` by default. Override it with:

```powershell
$env:PORT=3000
cargo run
```

## Endpoints

| Method | Path | Purpose |
| --- | --- | --- |
| `GET` | `/health` | Runtime health check for deployment platforms. |
| `GET` | `/api/profile` | Candidate/project profile with Rust, Go, Node.js, and NestJS positioning. |
| `GET` | `/api/signals` | Default engineering signals demonstrated by the app. |
| `POST` | `/api/signals/evaluate` | Scores custom evidence for a target role. |
| `GET` | `/api/pulse` | Default CargoPulse service health report. |
| `POST` | `/api/pulse/evaluate` | Evaluates custom service checks. |

Example signal request:

```json
{
  "role": "backend engineer",
  "evidence": [
    {
      "name": "Rust performance",
      "level": "production",
      "evidence": "Built an async Rust API with typed routes, modular tests, and operational health responses."
    },
    {
      "name": "deploy readiness",
      "level": "strong",
      "evidence": "Documented runtime configuration, health checks, and rollback notes for deployment."
    }
  ]
}
```

Example pulse request:

```json
{
  "service": "portfolio-api",
  "checks": [
    { "name": "http", "success": true, "latency_ms": 22, "critical": true },
    { "name": "database", "success": false, "latency_ms": 0, "critical": true },
    { "name": "docs", "success": true, "latency_ms": 5, "critical": false }
  ]
}
```

## What To Tell Recruiters

"I built `rust-signalforge` as a compact Rust backend that shows how I think about reliable services. It has async HTTP routes with Axum and Tokio, typed models with Serde, pure scoring logic that is easy to test, health endpoints for deployment, and integration tests around the API. The project is intentionally small, but the structure mirrors how I would build production services: clear boundaries, observable runtime behavior, and documentation that explains trade-offs."

## Rust vs Node.js vs NestJS

**Rust** gives compile-time memory safety, predictable performance, and strong type guarantees without a garbage collector. It is excellent when reliability, latency, concurrency, and resource usage matter. The trade-off is a steeper learning curve and slower initial development for simple CRUD apps.

**Node.js** is fast to ship, has a huge ecosystem, and is very productive for I/O-heavy APIs. It uses JavaScript or TypeScript on a single-threaded event loop model. It is easier to start with, but runtime errors and dependency sprawl can become risks if the project is not disciplined.

**NestJS** is a structured framework on top of Node.js. It gives modules, controllers, providers, dependency injection, guards, pipes, and decorators. It feels closer to Angular or enterprise backend frameworks. NestJS is great for team consistency and large TypeScript APIs, while Rust is stronger when low-level control, performance, and compile-time guarantees matter more.

The short version: **Node.js is quick and flexible, NestJS is structured TypeScript backend development, and Rust is for highly reliable, high-performance systems where correctness matters before runtime.**

## Verification

Expected checks:

```powershell
cargo fmt --check
cargo test
cargo build
```

## Deployment Notes

- Runtime command: `cargo run --release`
- Build command: `cargo build --release`
- Required environment variables: none
- Optional environment variables:
  - `PORT`: HTTP port, defaults to `8080`
  - `RUST_LOG`: tracing filter, defaults to `rust_signalforge=info,tower_http=info`
- Health check: `GET /health`

## License

MIT
