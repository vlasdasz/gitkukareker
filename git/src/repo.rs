use std::path::Path;

use anyhow::Result;
use git2::StatusOptions;

use crate::{Change, commit_history::CommitHistory};

pub struct Repo {
    repo: git2::Repository,
}

impl Repo {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            repo: git2::Repository::discover(path)?,
        })
    }

    pub fn current_branch(&self) -> Result<String> {
        let head = self.repo.head()?;
        Ok(if head.is_branch() {
            head.shorthand().unwrap_or("INVALID STRING")
        } else {
            "HEAD"
        }
        .to_string())
    }

    pub fn changes(&self) -> Result<Vec<Change>> {
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);

        let statuses = self.repo.statuses(Some(&mut status_opts))?;

        Ok(statuses.into_iter().map(Into::into).collect())
    }

    pub fn history(&self) -> Result<Vec<CommitHistory>> {
        let head = self.repo.head()?;
        let head_commit = head.peel_to_commit()?;

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push(head_commit.id())?;

        Ok(revwalk
            .map(|commit_id| self.repo.find_commit(commit_id.unwrap()).unwrap().into())
            .collect())
    }

    pub fn remote(&self) -> Result<String> {
        let binding = self.repo.remotes()?;
        let remotes: Vec<_> = binding.into_iter().collect();

        if remotes.len() != 1 {
            // TODO:
            panic!("Check why and how to handle it");
        }

        Ok(remotes.first().unwrap().unwrap_or("INVALID STRING").to_string())
    }
}
