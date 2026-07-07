use crate::models::{PulseCheck, PulseReport, PulseRequest, PulseStatus};

pub fn default_pulse() -> PulseReport {
    evaluate_pulse(PulseRequest {
        service: "rust-signalforge".into(),
        checks: vec![
            PulseCheck {
                name: "http_api".into(),
                success: true,
                latency_ms: 18,
                critical: true,
            },
            PulseCheck {
                name: "docs".into(),
                success: true,
                latency_ms: 3,
                critical: false,
            },
            PulseCheck {
                name: "tests".into(),
                success: true,
                latency_ms: 42,
                critical: true,
            },
        ],
    })
}

pub fn evaluate_pulse(request: PulseRequest) -> PulseReport {
    let total = request.checks.len();
    let successful = request.checks.iter().filter(|check| check.success).count();
    let failing_checks = failing_check_names(&request.checks);
    let average_latency_ms = average_latency(&request.checks);
    let has_critical_failure = request
        .checks
        .iter()
        .any(|check| check.critical && !check.success);

    let success_rate = if total == 0 {
        0.0
    } else {
        successful as f32 / total as f32
    };

    let status = if total == 0 || has_critical_failure {
        PulseStatus::Critical
    } else if success_rate < 1.0 || average_latency_ms > 250 {
        PulseStatus::Degraded
    } else {
        PulseStatus::Healthy
    };

    let recommendation = match status {
        PulseStatus::Healthy => "Service is ready to show: checks pass and latency is controlled.",
        PulseStatus::Degraded => {
            "Investigate failing optional checks or latency before a live demo."
        }
        PulseStatus::Critical => "Fix critical checks before presenting this as production-ready.",
    }
    .into();

    PulseReport {
        service: request.service,
        status,
        success_rate,
        average_latency_ms,
        failing_checks,
        recommendation,
    }
}

fn failing_check_names(checks: &[PulseCheck]) -> Vec<String> {
    checks
        .iter()
        .filter(|check| !check.success)
        .map(|check| check.name.clone())
        .collect()
}

fn average_latency(checks: &[PulseCheck]) -> u64 {
    if checks.is_empty() {
        return 0;
    }

    checks.iter().map(|check| check.latency_ms).sum::<u64>() / checks.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn critical_failure_marks_service_critical() {
        let report = evaluate_pulse(PulseRequest {
            service: "candidate-api".into(),
            checks: vec![
                PulseCheck {
                    name: "api".into(),
                    success: true,
                    latency_ms: 20,
                    critical: true,
                },
                PulseCheck {
                    name: "database".into(),
                    success: false,
                    latency_ms: 0,
                    critical: true,
                },
            ],
        });

        assert_eq!(report.status, PulseStatus::Critical);
        assert_eq!(report.failing_checks, vec!["database"]);
        assert_eq!(report.average_latency_ms, 10);
    }

    #[test]
    fn high_latency_degrades_otherwise_successful_service() {
        let report = evaluate_pulse(PulseRequest {
            service: "candidate-api".into(),
            checks: vec![PulseCheck {
                name: "api".into(),
                success: true,
                latency_ms: 320,
                critical: true,
            }],
        });

        assert_eq!(report.status, PulseStatus::Degraded);
        assert_eq!(report.success_rate, 1.0);
    }
}
