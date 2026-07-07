use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use rust_signalforge::{
    app,
    models::{PulseReport, PulseStatus, Readiness, SignalReport},
};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn health_returns_service_status() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let payload: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(payload["service"], "rust-signalforge");
    assert_eq!(payload["status"], "ok");
}

#[tokio::test]
async fn signal_evaluation_scores_recruiter_evidence() {
    let body = json!({
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
    });

    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/signals/evaluate")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let report: SignalReport = serde_json::from_slice(&body).unwrap();

    assert_eq!(report.readiness, Readiness::StrongSignal);
    assert!(report.score >= 70);
    assert!(report
        .highlights
        .iter()
        .any(|item| item.contains("Rust performance")));
}

#[tokio::test]
async fn pulse_evaluation_detects_critical_failures() {
    let body = json!({
        "service": "portfolio-api",
        "checks": [
            { "name": "http", "success": true, "latency_ms": 22, "critical": true },
            { "name": "database", "success": false, "latency_ms": 0, "critical": true },
            { "name": "docs", "success": true, "latency_ms": 5, "critical": false }
        ]
    });

    let response = app()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/pulse/evaluate")
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let report: PulseReport = serde_json::from_slice(&body).unwrap();

    assert_eq!(report.status, PulseStatus::Critical);
    assert_eq!(report.failing_checks, vec!["database"]);
    assert!(report.recommendation.contains("critical"));
}
