use inquire::Select;

use crate::git_operations;

pub fn run_reset() -> Result<(), String> {
    let commit_log = git_operations::get_log()?;
    let selected_commit = Select::new("Select commit to rebase:", commit_log)
        .prompt()
        .map_err(|e| format!("Failed to rebase commit: {}", e))?;

    Ok(())
}
