use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RiskThresholds {
    pub review: f64,
    pub confirm: f64,
    pub block: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub db_path: PathBuf,
    pub worktree_root: PathBuf,
    pub max_parallel_sessions: usize,
    pub max_parallel_worktrees: usize,
    pub session_timeout_secs: u64,
    pub heartbeat_interval_secs: u64,
    pub default_agent: String,
    pub theme: Theme,
    pub risk_thresholds: RiskThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        Self {
            db_path: home.join(".claude").join("ecc2.db"),
            worktree_root: PathBuf::from("/tmp/ecc-worktrees"),
            max_parallel_sessions: 8,
            max_parallel_worktrees: 6,
            session_timeout_secs: 3600,
            heartbeat_interval_secs: 30,
            default_agent: "claude".to_string(),
            theme: Theme::Dark,
            risk_thresholds: Self::RISK_THRESHOLDS,
        }
    }
}

impl Config {
    pub const RISK_THRESHOLDS: RiskThresholds = RiskThresholds {
        review: 0.35,
        confirm: 0.60,
        block: 0.85,
    };

    pub fn load() -> Result<Self> {
        let config_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".claude")
            .join("ecc2.toml");

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Config::default())
        }
    }
}

impl Default for RiskThresholds {
    fn default() -> Self {
        Config::RISK_THRESHOLDS
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn default_config_uses_default_risk_thresholds() {
        let config = Config::default();

        assert_eq!(config.risk_thresholds, Config::RISK_THRESHOLDS);
    }

    #[test]
    fn deserialization_defaults_risk_thresholds() {
        let config: Config = toml::from_str(
            r#"
db_path = "/tmp/ecc2.db"
worktree_root = "/tmp/ecc-worktrees"
max_parallel_sessions = 8
max_parallel_worktrees = 6
session_timeout_secs = 3600
heartbeat_interval_secs = 30
default_agent = "claude"
theme = "Dark"
"#,
        )
        .expect("config should deserialize");

        assert_eq!(config.risk_thresholds, Config::RISK_THRESHOLDS);
    }
}
