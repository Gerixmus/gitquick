use std::{fmt, process::Command};

use inquire::{Confirm, Select};

struct Commit {
    hash: String,
    message: String,
}

impl fmt::Display for Commit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn run_revert() -> Result<(), String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=format:%H%x00%s")
        .output()
        .map_err(|e| format!("Failed to log messages: {}", e))?;
    let commits = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| {
            let data: Vec<&str> = s.split('\0').collect();
            Commit {
                hash: data[0].to_string(),
                message: data[1].to_string(),
            }
        })
        .collect();

    let selected_commit = Select::new("Select commit to revert:", commits)
        .prompt()
        .map_err(|e| format!("Failed to revert commit: {}", e))?;
    let message = format!(
        "revert: \"{}\"\nThis reverts commit: {}",
        selected_commit.message, selected_commit.hash
    );
    print_in_box(&message);

    let should_commit = Confirm::new("Revert?")
        .with_default(true)
        .prompt()
        .map_err(|e| format!("Failed to get confirmation: {}", e))?;

    if should_commit {
        Command::new("git")
            .arg("revert")
            .arg("--no-commit")
            .arg(&selected_commit.hash)
            .output()
            .map_err(|e| format!("Failed to revert: {}", e))?;
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .map_err(|e| format!("Failed to commit: {}", e))?;
        println!("✅ Revert successful!");
    } else {
        println!("❌ Revert canceled or failed to get user confirmation.");
    }

    Ok(())
}

fn print_in_box(message: &str) {
    let lines: Vec<&str> = message.lines().collect();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    println!("┌{}┐", "─".repeat(max_len + 2));
    for line in lines {
        println!("│ {:width$} │", line, width = max_len);
    }
    println!("└{}┘", "─".repeat(max_len + 2));
}
