use inquire::{Confirm, Select};

use crate::{
    commit::print_in_box,
    git_operations::{commit, get_log, revert},
};

pub fn run_revert() -> Result<(), String> {
    let commits = get_log()?;

    let selected_commit = Select::new("Select commit to revert:", commits)
        .prompt()
        .map_err(|e| format!("Failed to revert commit: {}", e))?;
    let message = format!(
        "revert: \"{}\"\nThis reverts commit: {}",
        selected_commit.message, selected_commit.hash
    );
    print_in_box(&message).map_err(|e| format!("Formatting failed: {}", e))?;

    let should_commit = Confirm::new("Revert?")
        .with_default(true)
        .prompt()
        .map_err(|e| format!("Failed to get confirmation: {}", e))?;

    if should_commit {
        revert(&selected_commit.hash).map_err(|e| format!("Failed to revert: {}", e))?;
        commit(&message, false).map_err(|e| format!("Failed to commit: {}", e))?;
        println!("✅ Revert successful!");
    } else {
        println!("❌ Revert canceled or failed to get user confirmation.");
    }

    Ok(())
}
