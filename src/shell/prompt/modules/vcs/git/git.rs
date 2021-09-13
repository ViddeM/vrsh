use crate::shell::common::colors::{fg_color, reset_color, Color};
use crate::shell::prompt::modules::vcs::git::git::GitError::InvalidBranchName;
use git2::{Branch, BranchType, Error, Repository, Status};
use std::env::current_dir;
use std::fmt::{Display, Formatter};
use std::{fmt, io};

pub enum GitError {
    IOError(io::Error),
    Git2Error(git2::Error),
    InvalidBranchName(String),
}

impl Display for GitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GitError::IOError(e) => write!(f, "io error: '{}'", e),
            GitError::Git2Error(e) => write!(f, "git2 error: '{}'", e),
            GitError::InvalidBranchName(name) => write!(f, "invalid branch name '{}'", name),
        }
    }
}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> Self {
        GitError::IOError(err)
    }
}

impl From<git2::Error> for GitError {
    fn from(err: Error) -> Self {
        GitError::Git2Error(err)
    }
}

pub fn get_git_prompt() -> Result<String, GitError> {
    let curr_dir = current_dir()?;
    let repo = match Repository::discover(curr_dir) {
        Ok(repo) => repo,
        Err(_) => {
            // Not a git repo, return an empty string
            return Ok(String::from(""));
        }
    };

    let branch_info = get_branch_info(&repo)?;
    let status_info = get_status_info(&repo)?;
    // Put it all together and return.
    Ok(format!(" {}{}", branch_info, status_info))
}

fn get_branch_info(repo: &Repository) -> Result<String, GitError> {
    let branch_info = match repo.head() {
        Ok(val) => match val.name() {
            None => {
                // print  @<commit_name>
                let commit_hash = repo.head()?.peel_to_commit()?.id();
                format!("{} @{}", fg_color(Color::BrightGreen), commit_hash)
            }
            Some(name) => {
                let branch_name = match name.strip_prefix("refs/heads/") {
                    None => return Err(InvalidBranchName(String::from(name))),
                    Some(n) => n,
                };

                let branch = repo.find_branch(branch_name, BranchType::Local)?;
                let github = github_string(&branch, repo)?;
                let upstream_info = upstream_info(&branch, repo)?;

                format!(
                    "{}{} {} {}{}{}",
                    fg_color(Color::BrightBlue),
                    github,
                    fg_color(Color::BrightGreen),
                    branch_name,
                    upstream_info,
                    reset_color(),
                )
            }
        },
        _ => {
            // Assume that the error is caused by the repository being empty (no commits).
            format!("{} init", fg_color(Color::BrightGreen))
        }
    };

    if branch_info.is_empty() == false {
        return Ok(format!("{}on {}", fg_color(Color::Orange), branch_info));
    }

    Ok(branch_info)
}

fn github_string(branch: &Branch, repo: &Repository) -> Result<String, GitError> {
    Ok(match (match branch.upstream() {
        Ok(v) => v,
        Err(_) => return Ok(String::new()),
    })
    .name()?
    {
        None => "",
        Some(upstream_name) => match upstream_name.split("/").into_iter().next() {
            None => "",
            Some(remote_name) => {
                let remote = repo.find_remote(remote_name)?;
                match remote.url() {
                    None => "",
                    Some(url) => match url.contains("github.com") {
                        true => "",
                        false => "",
                    },
                }
            }
        },
    }
    .to_string())
}

fn upstream_info(branch: &Branch, repo: &Repository) -> Result<String, GitError> {
    let local_object = repo.revparse_single(match branch.name()? {
        Some(v) => v,
        None => return Ok(String::new()),
    })?;
    let local_commit = local_object.peel_to_commit()?;

    let remote_object = repo.revparse_single(
        match (match branch.upstream() {
            Ok(v) => v,                         // The branch has a remote
            Err(_) => return Ok(String::new()), // The branch doesn't have a remote
        })
        .name()?
        {
            Some(v) => v,
            None => return Ok(String::new()),
        },
    )?;
    let remote_commit = remote_object.peel_to_commit()?;

    let (ahead, behind) = repo.graph_ahead_behind(local_commit.id(), remote_commit.id())?;
    let ahead_string = if ahead > 0 {
        format!(" ↑{}", ahead)
    } else {
        String::new()
    };

    let behind_string = if behind > 0 {
        format!(" ↓{}", behind)
    } else {
        String::new()
    };

    Ok(format!("{}{}", ahead_string, behind_string))
}

fn get_status_info(repo: &Repository) -> Result<String, GitError> {
    let mut num_untracked: u32 = 0;
    let mut num_unstaged: u32 = 0;
    let mut num_staged: u32 = 0;

    for statuses in repo.statuses(None).into_iter() {
        for status_entry in statuses.iter() {
            // println!("{} :: {:?}", s.path().unwrap_or(""), s.status());
            let status = status_entry.status();
            if status.is_wt_new() {
                num_untracked += 1;
            } else if is_modified(status) {
                num_unstaged += 1;
            } else if is_staged(status) {
                num_staged += 1;
            } else if !status.is_ignored() {
                eprintln!("Unhandled git status: {:?}", status);
            }
        }
    }

    let untracked = if num_untracked > 0 {
        format!(" {}?{}", fg_color(Color::Blue), num_untracked)
    } else {
        "".to_owned()
    };

    let unstaged = if num_unstaged > 0 {
        format!(" {}!{}", fg_color(Color::Yellow), num_unstaged)
    } else {
        "".to_owned()
    };

    let staged = if num_staged > 0 {
        format!(" {}+{}", fg_color(Color::Orange), num_staged)
    } else {
        "".to_owned()
    };

    Ok(format!("{}{}{}", staged, unstaged, untracked))
}

fn is_modified(status: Status) -> bool {
    return status.is_wt_deleted()
        || status.is_wt_modified()
        || status.is_wt_renamed()
        || status.is_wt_typechange()
        || status.is_wt_new();
}

fn is_staged(status: Status) -> bool {
    return status.is_index_deleted()
        || status.is_index_modified()
        || status.is_index_new()
        || status.is_index_renamed()
        || status.is_index_typechange();
}
