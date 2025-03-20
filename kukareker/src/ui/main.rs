use std::path::PathBuf;

use git2::{Repository, StatusOptions};
use rfd::FileDialog;
use test_engine::{
    refs::Weak,
    ui::{
        link_button, view,
        Anchor::{Top, X},
        Button, DropDown, HasText, Setup, ViewData,
    },
    Window,
};

use crate::{
    model::State,
    ui::{
        changes::Changes,
        history::{CommitHistory, History},
    },
};

#[view]
pub struct Main {
    #[init]
    repo_name: DropDown<PathBuf>,
    open:      Button,

    changes: Changes,

    history: History,
}

impl Setup for Main {
    fn setup(mut self: Weak<Self>) {
        Window::set_title("GitKukareker");

        self.repo_name.place().tl(20).size(400, 100);
        self.repo_name
            .custom_format(|path| path.file_name().unwrap().to_string_lossy().to_string());
        self.repo_name.on_changed(move |path| {
            self.repo_selected(&path).unwrap();
        });

        self.open.set_text("Open");
        self.open
            .place()
            .same([X], self.repo_name)
            .anchor(Top, self.repo_name, 20)
            .size(200, 100);

        link_button!(self, open, on_open);

        self.changes.place().trb(0).w(500);

        self.history.place().t(200).lrb(0);

        self.update();
    }
}

impl Main {
    fn update(mut self: Weak<Self>) {
        self.repo_name.set_values(State::repos().collect());
    }

    fn on_open(self: Weak<Self>) {
        let Some(result) = FileDialog::new().set_directory("/").pick_folder() else {
            return;
        };

        State::add_repo(result);
        self.update();
    }

    fn repo_selected(mut self: Weak<Self>, path: &PathBuf) -> anyhow::Result<()> {
        let Ok(repo) = Repository::discover(path) else {
            panic!("no repo")
        };

        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);

        let statuses = repo.statuses(Some(&mut status_opts))?;

        self.changes.set_changes(statuses.into_iter().map(Into::into).collect());

        let head = repo.head()?;
        let head_commit = head.peel_to_commit()?;

        let mut revwalk = repo.revwalk()?;
        revwalk.push(head_commit.id())?;

        let history: Vec<CommitHistory> = revwalk
            .map(|commit_id| repo.find_commit(commit_id.unwrap()).unwrap().into())
            .collect();

        self.history.set_history(history);

        Ok(())
    }
}
