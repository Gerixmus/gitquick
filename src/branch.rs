use inquire::{Confirm, MultiSelect};

use crate::git_operations::{self, delete_branch, Branch};

pub fn run_branch(delete: bool, force_delete: bool) -> Result<(), String> {
    let branches = git_operations::get_branches().map_err(|e| e.to_string())?;
    if delete || force_delete {
        let branches: Vec<Branch> = branches.into_iter().filter(|branch| !branch.head).collect();

        let selected_branches = MultiSelect::new("Select branches to delete", branches)
            .prompt()
            .map_err(|e| format!("Prompt error: {}", e))?;

        let flag = if delete { "-d" } else { "-D" };

        let should_delete = Confirm::new("Delete selected branches?")
            .with_default(true)
            .prompt()
            .map_err(|e| format!("Failed to get confirmation: {}", e))?;

        if should_delete {
            for branch in selected_branches {
                delete_branch(&branch.name, flag)
                    .map_err(|e| format!("Failed to delete branch: {}", e))?;
            }
        } else {
            println!("❌ Commit canceled or failed to get user confirmation.");
        }

        Ok(())
    } else {
        branches.iter().for_each(|branch| println!("{branch}"));
        Ok(())
    }
}
