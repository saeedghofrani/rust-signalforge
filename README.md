# rust-signalforge

`rust-signalforge` is a small Rust/Axum API for evaluating supplied engineering evidence and service-health samples. It combines two demonstrations:

- **SignalForge:** turns project evidence into a recruiter-readable engineering signal report.
- **CargoPulse:** evaluates service health checks, latency, and critical failures.

The app is intentionally small, but it is built like a real backend service: typed request/response models, async routes, pure domain logic, integration tests, health checks, structured logging, and deployment-friendly configuration.

## Why This Exists

The project explores how to keep HTTP transport separate from deterministic application logic. Its small scope makes these choices easy to inspect:

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

## Design and limitations

- Axum handlers accept typed Serde payloads; scoring logic is separate from routing so domain and HTTP behavior can be tested independently.
- Tokio provides the async runtime; structured tracing gives requests an operational trail.
- The health-evaluation endpoints score **supplied samples**. They do not poll external services or establish real uptime.
- Engineering scores are demonstration heuristics, not an automated hiring assessment or proof of professional experience.
- This is a learning project, not evidence of production Rust ownership. It does not demonstrate durable persistence, authentication, or production load behavior.
- Deployment readiness still depends on environment-specific monitoring, access controls, and operational validation.

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
