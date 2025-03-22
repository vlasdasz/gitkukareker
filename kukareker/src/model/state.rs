use std::{collections::BTreeSet, path::PathBuf};

use test_engine::store::OnDisk;

static REPOS: OnDisk<BTreeSet<PathBuf>> = OnDisk::new("repos");

pub struct State {}

impl State {
    pub fn repos() -> impl Iterator<Item = PathBuf> {
        REPOS.get().into_iter()
    }

    pub fn add_repo(repo: PathBuf) {
        let mut repos = REPOS.get();
        repos.insert(repo);
        REPOS.set(repos);
    }
}
