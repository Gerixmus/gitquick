use inquire::MultiSelect;

use crate::git_operations::{self, Change};

pub fn run_stash(push: bool) -> Result<(), String> {
    if push {
        let repo = git_operations::get_repository().map_err(|e| e.to_string())?;

        let (changes, _staged) = git_operations::get_changes(&repo);

        if changes.is_empty() {
            println!("No untracked or modified files found.");
            return Ok(());
        }

        let repo = git_operations::get_repository().map_err(|e| e.to_string())?;

        let (changes, _staged) = git_operations::get_changes(&repo);

        if changes.is_empty() {
            println!("No untracked or modified files found.");
            return Ok(());
        }

        let mut selected_files = Vec::<Change>::new();

        let selected_unstaged = MultiSelect::new("Select changes to stash:", changes)
            .prompt()
            .map_err(|e| format!("An error occurred during selection: {}", e))?;

        if selected_unstaged.is_empty() && selected_files.is_empty() {
            println!("No files selected.");
            return Ok(());
        }

        selected_files.extend(selected_unstaged);

        let _ = git_operations::push_stash(selected_files)
            .map_err(|e| format!("An error occurred during stash: {}", e))?;
    } else {
    }
    Ok(())
}
