//! Beads integration — reads issue state for display.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::process::Command;

/// A Beads issue for display.
#[derive(Debug, Clone, Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub status: String,
    #[serde(default)]
    pub description: Option<String>,
    pub priority: u8,
    pub issue_type: String,
    #[serde(default)]
    pub assignee: Option<String>,
}

/// Fetch all issues from Beads.
pub fn list_issues() -> Result<Vec<Issue>> {
    let output = Command::new("bd")
        .args(["list", "--json"])
        .output()
        .context("failed to run `bd list`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("`bd list` failed: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let issues: Vec<Issue> = serde_json::from_str(&stdout)
        .context("failed to parse `bd list` output")?;

    Ok(issues)
}

/// Fetch ready (unblocked) issues.
pub fn ready_issues() -> Result<Vec<Issue>> {
    let output = Command::new("bd")
        .args(["ready", "--json"])
        .output()
        .context("failed to run `bd ready`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("`bd ready` failed: {stderr}");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let issues: Vec<Issue> = serde_json::from_str(&stdout)
        .context("failed to parse `bd ready` output")?;

    Ok(issues)
}
