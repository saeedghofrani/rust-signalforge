use axum::{routing::get, routing::post, Json, Router};
use serde_json::json;
use tower_http::trace::TraceLayer;

use crate::{
    models::{PulseRequest, SignalRequest},
    pulse::{default_pulse, evaluate_pulse},
    scoring::{default_signals, evaluate_signal, showcase_profile},
};

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/profile", get(profile))
        .route("/api/signals", get(signals))
        .route("/api/signals/evaluate", post(evaluate_signals))
        .route("/api/pulse", get(pulse))
        .route("/api/pulse/evaluate", post(evaluate_pulse_report))
        .layer(TraceLayer::new_for_http())
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "service": "rust-signalforge",
        "status": "ok",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

async fn profile() -> Json<crate::models::Profile> {
    Json(showcase_profile())
}

async fn signals() -> Json<Vec<crate::models::SkillSignal>> {
    Json(default_signals())
}

async fn evaluate_signals(Json(request): Json<SignalRequest>) -> Json<crate::models::SignalReport> {
    Json(evaluate_signal(request))
}

async fn pulse() -> Json<crate::models::PulseReport> {
    Json(default_pulse())
}

async fn evaluate_pulse_report(
    Json(request): Json<PulseRequest>,
) -> Json<crate::models::PulseReport> {
    Json(evaluate_pulse(request))
}
