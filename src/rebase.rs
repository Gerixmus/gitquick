use std::process::{Command, Stdio};

use inquire::Select;

use crate::git_operations::get_log;

pub fn run_rebase(interactive: bool) -> Result<(), String> {
    let mut rebase_command = Command::new("git");
    rebase_command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .arg("rebase")
        .arg("--autosquash");

    if interactive {
        rebase_command.arg("--interactive");
    }

    let commit_log = get_log()?;
    let selected_commit = Select::new("Select commit to revert:", commit_log)
        .prompt()
        .map_err(|e| format!("Failed to revert commit: {}", e))?;

    rebase_command.arg(selected_commit.hash);
    rebase_command
        .status()
        .map_err(|e| format!("Failed to rebase: {}", e))?;

    Ok(())
}
