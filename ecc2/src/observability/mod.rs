use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::config::RiskThresholds;
use crate::session::store::StateStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallEvent {
    pub session_id: String,
    pub tool_name: String,
    pub input_summary: String,
    pub output_summary: String,
    pub duration_ms: u64,
    pub risk_score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub score: f64,
    pub reasons: Vec<String>,
    pub suggested_action: SuggestedAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestedAction {
    Allow,
    Review,
    RequireConfirmation,
    Block,
}

impl ToolCallEvent {
    /// Compute risk from the tool type and input characteristics.
    pub fn compute_risk(
        tool_name: &str,
        input: &str,
        thresholds: &RiskThresholds,
    ) -> RiskAssessment {
        let normalized_tool = tool_name.to_ascii_lowercase();
        let normalized_input = input.to_ascii_lowercase();
        let mut score = 0.0;
        let mut reasons = Vec::new();

        let (base_score, base_reason) = base_tool_risk(&normalized_tool);
        score += base_score;
        if let Some(reason) = base_reason {
            reasons.push(reason.to_string());
        }

        let (file_sensitivity_score, file_sensitivity_reason) =
            assess_file_sensitivity(&normalized_input);
        score += file_sensitivity_score;
        if let Some(reason) = file_sensitivity_reason {
            reasons.push(reason);
        }

        let (blast_radius_score, blast_radius_reason) = assess_blast_radius(&normalized_input);
        score += blast_radius_score;
        if let Some(reason) = blast_radius_reason {
            reasons.push(reason);
        }

        let (irreversibility_score, irreversibility_reason) =
            assess_irreversibility(&normalized_input);
        score += irreversibility_score;
        if let Some(reason) = irreversibility_reason {
            reasons.push(reason);
        }

        let score = score.clamp(0.0, 1.0);
        let suggested_action = SuggestedAction::from_score(score, thresholds);

        RiskAssessment {
            score,
            reasons,
            suggested_action,
        }
    }
}

impl SuggestedAction {
    fn from_score(score: f64, thresholds: &RiskThresholds) -> Self {
        if score >= thresholds.block {
            Self::Block
        } else if score >= thresholds.confirm {
            Self::RequireConfirmation
        } else if score >= thresholds.review {
            Self::Review
        } else {
            Self::Allow
        }
    }
}

fn base_tool_risk(tool_name: &str) -> (f64, Option<&'static str>) {
    match tool_name {
        "bash" => (
            0.20,
            Some("shell execution can modify local or shared state"),
        ),
        "write" | "multiedit" => (0.15, Some("writes files directly")),
        "edit" => (0.10, Some("modifies existing files")),
        _ => (0.05, None),
    }
}

fn assess_file_sensitivity(input: &str) -> (f64, Option<String>) {
    const SECRET_PATTERNS: &[&str] = &[
        ".env",
        "secret",
        "credential",
        "token",
        "api_key",
        "apikey",
        "auth",
        "id_rsa",
        ".pem",
        ".key",
    ];
    const SHARED_INFRA_PATTERNS: &[&str] = &[
        "cargo.toml",
        "package.json",
        "dockerfile",
        ".github/workflows",
        "schema",
        "migration",
        "production",
    ];

    if contains_any(input, SECRET_PATTERNS) {
        (
            0.25,
            Some("targets a sensitive file or credential surface".to_string()),
        )
    } else if contains_any(input, SHARED_INFRA_PATTERNS) {
        (
            0.15,
            Some("targets shared infrastructure or release-critical files".to_string()),
        )
    } else {
        (0.0, None)
    }
}

fn assess_blast_radius(input: &str) -> (f64, Option<String>) {
    const LARGE_SCOPE_PATTERNS: &[&str] = &[
        "**",
        "/*",
        "--all",
        "--recursive",
        "entire repo",
        "all files",
        "across src/",
        "find ",
        " xargs ",
    ];
    const SHARED_STATE_PATTERNS: &[&str] = &[
        "git push --force",
        "git push -f",
        "origin main",
        "origin master",
        "rm -rf .",
        "rm -rf /",
    ];

    if contains_any(input, SHARED_STATE_PATTERNS) {
        (
            0.35,
            Some("has a broad blast radius across shared state or history".to_string()),
        )
    } else if contains_any(input, LARGE_SCOPE_PATTERNS) {
        (
            0.25,
            Some("has a broad blast radius across multiple files or directories".to_string()),
        )
    } else {
        (0.0, None)
    }
}

fn assess_irreversibility(input: &str) -> (f64, Option<String>) {
    const HIGH_IRREVERSIBILITY_PATTERNS: &[&str] = &[
        "rm -rf",
        "git reset --hard",
        "git clean -fd",
        "drop database",
        "drop table",
        "truncate ",
        "shred ",
    ];
    const MODERATE_IRREVERSIBILITY_PATTERNS: &[&str] =
        &["rm -f", "git push --force", "git push -f", "delete from"];

    if contains_any(input, HIGH_IRREVERSIBILITY_PATTERNS) {
        (
            0.45,
            Some("includes an irreversible or destructive operation".to_string()),
        )
    } else if contains_any(input, MODERATE_IRREVERSIBILITY_PATTERNS) {
        (
            0.40,
            Some("includes an irreversible or difficult-to-undo operation".to_string()),
        )
    } else {
        (0.0, None)
    }
}

fn contains_any(input: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|pattern| input.contains(pattern))
}

pub fn log_tool_call(db: &StateStore, event: &ToolCallEvent) -> Result<()> {
    db.send_message(
        &event.session_id,
        "observability",
        &serde_json::to_string(event)?,
        "tool_call",
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{SuggestedAction, ToolCallEvent};
    use crate::config::Config;

    #[test]
    fn computes_sensitive_file_risk() {
        let assessment = ToolCallEvent::compute_risk(
            "Write",
            "Update .env.production with rotated API token",
            &Config::RISK_THRESHOLDS,
        );

        assert!(assessment.score >= Config::RISK_THRESHOLDS.review);
        assert_eq!(assessment.suggested_action, SuggestedAction::Review);
        assert!(assessment
            .reasons
            .iter()
            .any(|reason| reason.contains("sensitive file")));
    }

    #[test]
    fn computes_blast_radius_risk() {
        let assessment = ToolCallEvent::compute_risk(
            "Edit",
            "Apply the same replacement across src/**/*.rs",
            &Config::RISK_THRESHOLDS,
        );

        assert!(assessment.score >= Config::RISK_THRESHOLDS.review);
        assert_eq!(assessment.suggested_action, SuggestedAction::Review);
        assert!(assessment
            .reasons
            .iter()
            .any(|reason| reason.contains("blast radius")));
    }

    #[test]
    fn computes_irreversible_risk() {
        let assessment = ToolCallEvent::compute_risk(
            "Bash",
            "rm -f /tmp/ecc-temp.txt",
            &Config::RISK_THRESHOLDS,
        );

        assert!(assessment.score >= Config::RISK_THRESHOLDS.confirm);
        assert_eq!(
            assessment.suggested_action,
            SuggestedAction::RequireConfirmation,
        );
        assert!(assessment
            .reasons
            .iter()
            .any(|reason| reason.contains("irreversible")));
    }

    #[test]
    fn blocks_combined_high_risk_operations() {
        let assessment = ToolCallEvent::compute_risk(
            "Bash",
            "rm -rf . && git push --force origin main",
            &Config::RISK_THRESHOLDS,
        );

        assert!(assessment.score >= Config::RISK_THRESHOLDS.block);
        assert_eq!(assessment.suggested_action, SuggestedAction::Block);
    }
}
