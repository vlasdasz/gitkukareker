use std::path::Path;

use anyhow::Result;
use git2::{
    AutotagOption, Cred, FetchOptions, IndexAddOption, PushOptions, RemoteCallbacks, RemoteUpdateFlags,
    ResetType, StatusOptions,
};

use crate::{Change, commit_history::CommitHistory, credentials::Credentials};

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

    pub fn stage_all(&self) -> Result<()> {
        let mut index = self.repo.index()?;
        index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
        index.write()?;

        Ok(())
    }

    pub fn has_staged_changes(&self) -> Result<bool> {
        Ok(self.changes()?.iter().any(|change| change.staged))
    }

    pub fn push(&self) -> Result<()> {
        let remote = self.remote()?;

        let mut remote = self.repo.find_remote(&remote)?;

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|url, username_from_url, allowed_types| {
            println!("Connecting to: {url}");

            dbg!(&username_from_url);
            dbg!(&allowed_types);

            if allowed_types.is_ssh_key() {
                return Ok(Credentials::get().unwrap());
            }

            Err(git2::Error::from_str("No valid credentials available"))
        });

        callbacks.certificate_check(|_, _| Ok(git2::CertificateCheckStatus::CertificateOk));

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(callbacks);

        let branch = self.current_branch()?;

        let refspec = format!("refs/heads/{branch}:refs/heads/{branch}");
        remote.push(&[refspec], Some(&mut push_options))?;

        Ok(())
    }

    pub fn commit_and_push(&self, message: impl ToString) -> Result<()> {
        if !self.has_staged_changes()? {
            println!("Nothing to commit");
            self.push()?;
            return Ok(());
        }

        let mut index = self.repo.index()?;

        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let head = self.repo.head();
        let parent_commit = match head {
            Ok(head_ref) if head_ref.is_branch() => Some(head_ref.peel_to_commit()?),
            _ => None,
        };

        let sig = self.repo.signature()?;

        let message = message.to_string();

        if let Some(parent) = parent_commit {
            self.repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&parent])?
        } else {
            self.repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])? // First commit (no parent)
        };

        self.push()
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

    pub fn fetch(&self) -> Result<()> {
        let remote = self.remote()?;

        let mut remote = self.repo.find_remote(&remote)?;

        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|url, username_from_url, allowed_types| {
            println!("Connecting to: {url}");

            if allowed_types.is_ssh_key() {
                if allowed_types.is_ssh_key() {
                    dbg!("KRADE");
                    return Cred::ssh_key_from_agent(username_from_url.unwrap());
                }
            }

            Err(git2::Error::from_str("No valid credentials available"))
        });

        callbacks.certificate_check(|_cert, shmert| {
            // dbg!(&cert.as_hostkey().);

            dbg!("Check?");
            dbg!(&shmert);

            Ok(git2::CertificateCheckStatus::CertificateOk)
        });

        callbacks.push_update_reference(|a, b| {
            dbg!("push_update_reference");
            dbg!(&a);
            dbg!(&b);

            Ok(())
        });

        callbacks.push_negotiation(|a| {
            dbg!("FOROGORN");

            for a in a {
                dbg!(&a.src_refname());
            }

            Ok(())
        });

        callbacks.sideband_progress(|a| {
            dbg!("sideband_progress");
            dbg!(&a);
            true
        });

        callbacks.update_tips(|refname, a, b| {
            if a.is_zero() {
                println!("[new]     {:20} {}", b, refname);
            } else {
                println!("[updated] {:10}..{:10} {}", a, b, refname);
            }
            true
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        remote.fetch(
            &["refs/heads/*:refs/remotes/origin/*"],
            Some(&mut fetch_options),
            None,
        )?;

        // Disconnect the underlying connection to prevent from idling.
        remote.disconnect()?;

        // Update the references in the remote's namespace to point to the right
        // commits. This may be needed even if there was no packfile to download,
        // which can happen e.g. when the branches have been changed but all the
        // needed objects are available locally.
        remote.update_tips(
            None,
            RemoteUpdateFlags::UPDATE_FETCHHEAD,
            AutotagOption::Unspecified,
            None,
        )?;

        println!("Fetch completed!");

        Ok(())
    }

    pub fn discard_all(&self) -> Result<()> {
        let head_commit = self.repo.head()?.peel_to_commit()?;
        self.repo.reset(head_commit.as_object(), ResetType::Hard, None)?;

        // Discard untracked
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        let statuses = self.repo.statuses(Some(&mut status_opts))?;

        for entry in statuses.iter() {
            if entry.status().is_index_new() || entry.status().is_wt_new() {
                let full_path = self.repo.workdir().unwrap().join(entry.path().unwrap());
                std::fs::remove_file(&full_path)?;
            }
        }

        Ok(())
    }
}

impl Repo {
    fn remote(&self) -> Result<String> {
        let binding = self.repo.remotes()?;
        let remotes: Vec<_> = binding.into_iter().collect();

        if remotes.len() != 1 {
            // TODO:
            panic!("Check why and how to handle it");
        }

        Ok(remotes.first().unwrap().unwrap_or("INVALID STRING").to_string())
    }
}
