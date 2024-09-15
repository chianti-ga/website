use std::fs;
use std::process::Command;

fn main() {
    let git_commit = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let git_branch = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let build_timestamp = chrono::Utc::now().to_rfc3339();

    let git_tag = Command::new("git")
        .args(&["describe", "--tags"])
        .output()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    let build_info = serde_json::json!({
        "git_commit": git_commit,
        "git_branch": git_branch,
        "build_timestamp": build_timestamp,
        "git_tag": git_tag,
    });

    fs::write("build_info.json", build_info.to_string()).unwrap();
}
