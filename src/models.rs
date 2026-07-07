use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SkillSignal {
    pub name: String,
    pub level: SignalLevel,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignalLevel {
    Learning,
    Working,
    Strong,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectSignal {
    pub name: String,
    pub summary: String,
    pub stack: Vec<String>,
    pub signals: Vec<SkillSignal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub headline: String,
    pub projects: Vec<ProjectSignal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalRequest {
    pub role: String,
    pub evidence: Vec<SkillSignal>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignalReport {
    pub role: String,
    pub score: u8,
    pub readiness: Readiness,
    pub highlights: Vec<String>,
    pub next_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Readiness {
    Exploring,
    InterviewReady,
    StrongSignal,
    SeniorSignal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PulseCheck {
    pub name: String,
    pub success: bool,
    pub latency_ms: u64,
    pub critical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PulseRequest {
    pub service: String,
    pub checks: Vec<PulseCheck>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PulseReport {
    pub service: String,
    pub status: PulseStatus,
    pub success_rate: f32,
    pub average_latency_ms: u64,
    pub failing_checks: Vec<String>,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PulseStatus {
    Healthy,
    Degraded,
    Critical,
}
