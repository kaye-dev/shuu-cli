use std::process::{Command, Stdio};

pub struct Worktree {
    pub path: String,
    pub hash: String,
    pub branch: String,
}

pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--is-inside-work-tree"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn worktree_list() -> Vec<Worktree> {
    let output = match Command::new("git").args(["worktree", "list"]).output() {
        Ok(o) if o.status.success() => o,
        _ => return vec![],
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some(Worktree {
                    path: parts[0].to_string(),
                    hash: parts[1].to_string(),
                    branch: parts[2].trim_matches(|c| c == '[' || c == ']').to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn main_worktree() -> Option<String> {
    worktree_list().first().map(|wt| wt.path.clone())
}

pub fn worktrees_dir() -> Option<String> {
    let main = main_worktree()?;
    let path = std::path::Path::new(&main);
    let repo_name = path.file_name()?.to_str()?;
    let parent = path.parent()?.to_str()?;
    Some(format!("{}/{}-worktrees", parent, repo_name))
}

/// Returns Ok(true) if created with new branch, Ok(false) if existing branch, Err on failure
pub fn worktree_add(path: &str, branch: &str) -> Result<bool, ()> {
    // Try with -b (new branch)
    if let Ok(s) = Command::new("git")
        .args(["worktree", "add", "-b", branch, path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        if s.success() {
            return Ok(true);
        }
    }

    // Try without -b (existing branch)
    if let Ok(s) = Command::new("git")
        .args(["worktree", "add", path, branch])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
    {
        if s.success() {
            return Ok(false);
        }
    }

    Err(())
}

pub fn worktree_remove(path: &str) -> bool {
    Command::new("git")
        .args(["worktree", "remove", path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn worktree_force_remove(path: &str) -> bool {
    Command::new("git")
        .args(["worktree", "remove", "--force", path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn branch_delete(branch: &str) -> bool {
    Command::new("git")
        .args(["branch", "-d", branch])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn branch_force_delete(branch: &str) -> bool {
    Command::new("git")
        .args(["branch", "-D", branch])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

pub fn repo_name() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    std::path::Path::new(&path)
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
}

pub fn current_branch() -> Option<String> {
    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        None
    } else {
        Some(branch)
    }
}
