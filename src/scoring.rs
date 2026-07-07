use crate::models::{
    Profile, ProjectSignal, Readiness, SignalLevel, SignalReport, SignalRequest, SkillSignal,
};

pub fn showcase_profile() -> Profile {
    Profile {
        name: "Saeed Ghofrani".into(),
        headline: "Backend engineer building typed, reliable services across Rust, Go, Node.js, and NestJS.".into(),
        projects: vec![ProjectSignal {
            name: "rust-signalforge".into(),
            summary: "Async Rust API that turns project evidence and service health into recruiter-readable engineering signals.".into(),
            stack: vec![
                "Rust".into(),
                "Axum".into(),
                "Tokio".into(),
                "Serde".into(),
                "Tower".into(),
            ],
            signals: default_signals(),
        }],
    }
}

pub fn default_signals() -> Vec<SkillSignal> {
    vec![
        SkillSignal {
            name: "async Rust API design".into(),
            level: SignalLevel::Production,
            evidence: "Axum routes are backed by pure scoring modules and integration tests.".into(),
        },
        SkillSignal {
            name: "service reliability".into(),
            level: SignalLevel::Strong,
            evidence: "CargoPulse endpoints report health, latency, critical failures, and operational recommendations.".into(),
        },
        SkillSignal {
            name: "backend comparison thinking".into(),
            level: SignalLevel::Strong,
            evidence: "Documentation explains Rust trade-offs against Node.js and NestJS.".into(),
        },
    ]
}

pub fn evaluate_signal(request: SignalRequest) -> SignalReport {
    let raw_score: u16 = request
        .evidence
        .iter()
        .map(|signal| match signal.level {
            SignalLevel::Learning => 10,
            SignalLevel::Working => 20,
            SignalLevel::Strong => 30,
            SignalLevel::Production => 40,
        })
        .sum();

    let documentation_bonus = request
        .evidence
        .iter()
        .filter(|signal| signal.evidence.len() >= 48)
        .count() as u16
        * 4;
    let score = (raw_score + documentation_bonus).min(100) as u8;

    SignalReport {
        role: request.role,
        score,
        readiness: readiness_for(score),
        highlights: build_highlights(&request.evidence),
        next_steps: build_next_steps(score, &request.evidence),
    }
}

fn readiness_for(score: u8) -> Readiness {
    match score {
        0..=34 => Readiness::Exploring,
        35..=64 => Readiness::InterviewReady,
        65..=84 => Readiness::StrongSignal,
        _ => Readiness::SeniorSignal,
    }
}

fn build_highlights(evidence: &[SkillSignal]) -> Vec<String> {
    evidence
        .iter()
        .filter(|signal| matches!(signal.level, SignalLevel::Strong | SignalLevel::Production))
        .map(|signal| format!("{}: {}", signal.name, signal.evidence))
        .collect()
}

fn build_next_steps(score: u8, evidence: &[SkillSignal]) -> Vec<String> {
    let mut steps = Vec::new();

    if score < 65 {
        steps.push("Add one deployed project link with health checks and tests.".into());
    }

    if !evidence
        .iter()
        .any(|signal| signal.name.to_lowercase().contains("performance"))
    {
        steps.push("Add a small benchmark or latency note to prove performance awareness.".into());
    }

    if !evidence
        .iter()
        .any(|signal| signal.name.to_lowercase().contains("deploy"))
    {
        steps.push("Document deployment steps so recruiters see production readiness.".into());
    }

    if steps.is_empty() {
        steps.push("Keep the README focused on measurable impact and trade-offs.".into());
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_evidence_creates_senior_signal() {
        let report = evaluate_signal(SignalRequest {
            role: "backend engineer".into(),
            evidence: vec![
                SkillSignal {
                    name: "Rust performance".into(),
                    level: SignalLevel::Production,
                    evidence: "Built an async Rust API with typed request handling, tests, and clear latency-focused health responses.".into(),
                },
                SkillSignal {
                    name: "deploy readiness".into(),
                    level: SignalLevel::Production,
                    evidence: "Prepared runtime configuration, health endpoint, logging, and rollback notes for deployment.".into(),
                },
                SkillSignal {
                    name: "service ownership".into(),
                    level: SignalLevel::Strong,
                    evidence: "Designed the app as small modules that can be tested without running a server.".into(),
                },
            ],
        });

        assert_eq!(report.score, 100);
        assert_eq!(report.readiness, Readiness::SeniorSignal);
        assert_eq!(
            report.next_steps,
            vec!["Keep the README focused on measurable impact and trade-offs."]
        );
    }

    #[test]
    fn weak_evidence_produces_actionable_next_steps() {
        let report = evaluate_signal(SignalRequest {
            role: "backend engineer".into(),
            evidence: vec![SkillSignal {
                name: "Rust".into(),
                level: SignalLevel::Learning,
                evidence: "Learning syntax.".into(),
            }],
        });

        assert_eq!(report.readiness, Readiness::Exploring);
        assert!(report
            .next_steps
            .iter()
            .any(|step| step.contains("deployed project")));
        assert!(report
            .next_steps
            .iter()
            .any(|step| step.contains("benchmark")));
    }
}
