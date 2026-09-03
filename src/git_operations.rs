use core::fmt;
use git2::{Repository, Status, StatusOptions};
use std::{error::Error, process::Command};

#[derive(Clone)]
pub struct Change {
    pub path: String,
    status: git2::Status,
}

pub struct CommitLog {
    pub hash: String,
    pub message: String,
}

impl fmt::Display for CommitLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self.status {
            s if s.contains(Status::WT_NEW) => "new",
            s if s.contains(Status::WT_MODIFIED) => "modified",
            s if s.contains(Status::WT_DELETED) => "deleted",
            _ => "?",
        };
        write!(f, "{}: {}", status_str, self.path)
    }
}

pub fn get_log() -> Result<Vec<CommitLog>, String> {
    let output = Command::new("git")
        .arg("log")
        .arg("--pretty=format:%H%x00%s")
        .output()
        .map_err(|e| format!("Failed to log messages: {}", e))?;
    let commits = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| {
            let data: Vec<&str> = s.split('\0').collect();
            CommitLog {
                hash: data[0].to_string(),
                message: data[1].to_string(),
            }
        })
        .collect();
    Ok(commits)
}

pub fn push_stash(selected_files: Vec<Change>, message: &str) -> Result<(), Box<dyn Error>> {
    let paths: Vec<&String> = selected_files.iter().map(|change| &change.path).collect();
    let mut cmd = Command::new("git");
    cmd.arg("stash")
        .arg("push")
        .arg("--include-untracked")
        .args(paths);

    if !message.is_empty() {
        cmd.arg("--message").arg(message);
    }

    let _output = cmd
        .output()
        .map_err(|e| format!("Failed to stash files: {}", e))?;
    Ok(())
}

pub struct Branch {
    pub name: String,
    pub upstream: Option<String>,
    pub head: bool,
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(upstream) = &self.upstream {
            write!(f, " [{}]", upstream)?;
        }
        Ok(())
    }
}

pub fn get_branches() -> Result<Vec<Branch>, Box<dyn Error>> {
    let git_format = "%(refname:short)%00%(upstream:short)%00%(HEAD)";
    let output = Command::new("git")
        .arg("for-each-ref")
        .arg(format!("--format={}", git_format))
        .arg("refs/heads/")
        .arg("refs/remotes/")
        .output()?;

    let output_str = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let branches: Vec<Branch> = output_str
        .lines()
        .map(|l| {
            let info: Vec<&str> = l.split("\0").collect();
            Branch {
                name: info[0].to_owned(),
                upstream: (!info[1].is_empty()).then(|| info[1].to_owned()),
                head: info[2].is_empty(),
            }
        })
        .collect();
    Ok(branches)
}

#[allow(dead_code)]
fn fetch_with_prune() -> Result<(), std::io::Error> {
    let status = Command::new("git").arg("fetch").arg("--prune").status()?;

    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other("git fetch with prune failed"))
    }
}

pub fn get_repository() -> Result<Repository, git2::Error> {
    Repository::discover(".")
}

pub fn get_changes(repo: &Repository) -> (Vec<Change>, Vec<Change>) {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    status_opts.recurse_untracked_dirs(true);

    let statuses = match repo.statuses(Some(&mut status_opts)) {
        Ok(statuses) => statuses,
        Err(err) => {
            println!("Error fetching statuses: {}", err);
            return (Vec::new(), Vec::new());
        }
    };

    let mut untracked = Vec::new();
    let mut staged = Vec::new();

    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            let path = path.to_string();
            let status = entry.status();
            if status.intersects(Status::WT_NEW | Status::WT_MODIFIED | Status::WT_DELETED) {
                untracked.push(Change {
                    path: path.clone(),
                    status,
                });
            }
            if status.intersects(Status::INDEX_NEW | Status::INDEX_MODIFIED | Status::INDEX_DELETED)
            {
                staged.push(Change { path, status });
            }
        }
    }

    (untracked, staged)
}

pub fn add_files(selected_files: Vec<Change>) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .arg("add")
        .args(selected_files.iter().map(|f| &f.path))
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(Box::new(std::io::Error::other(err.to_string())))
    } else {
        Ok(())
    }
}

pub fn commit(message: &str, amend: bool) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("git");

    command.arg("commit");

    if amend {
        command.arg("--amend");
    };

    let output = command.arg("-m").arg(message).output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(Box::new(std::io::Error::other(err.to_string())))
    } else {
        Ok(())
    }
}

pub fn revert(hash: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git")
        .arg("revert")
        .arg("--no-commit")
        .arg(hash)
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(Box::new(std::io::Error::other(err.to_string())))
    } else {
        Ok(())
    }
}

pub fn checkout_branch(branch: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git").arg("checkout").arg(branch).output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(Box::new(std::io::Error::other(err.to_string())))
    } else {
        Ok(())
    }
}

pub fn get_current_branch() -> Result<String, git2::Error> {
    let repo = get_repository()?;

    let head = repo.head()?;
    head.shorthand()
        .map(|s| s.to_string())
        .ok_or_else(|| git2::Error::from_str("Failed to get branch name"))
}

pub fn create_and_checkout_branch(branch_name: &str) -> Result<(), git2::Error> {
    let repo = get_repository()?;

    let head_ref = repo.head()?;
    let target_commit = head_ref.peel_to_commit()?;

    let branch = repo.branch(branch_name, &target_commit, false)?;

    let branch_ref = branch
        .get()
        .name()
        .ok_or_else(|| git2::Error::from_str("Invalid branch reference name"))?;

    let obj = repo.revparse_single(branch_ref)?;

    repo.checkout_tree(&obj, None)?;
    repo.set_head(branch_ref)?;

    Ok(())
}
