use std::path::{Path, PathBuf};

use anyhow::Result;
use git::{Change, CommitHistory, Repo};
use rfd::FileDialog;
use rtools::Unwrap;
use test_engine::{
    Task, Window,
    refs::Weak,
    ui::{
        AlertErr,
        Anchor::{Top, X},
        Button, DropDown, HasText, Label, Setup, Spinner, ViewData, link_button, view,
    },
};

use crate::{
    model::State,
    ui::{changes::Changes, history::History},
};

#[view]
pub struct MainView {
    repo: Unwrap<Repo>,

    #[init]
    repo_name: DropDown<PathBuf>,
    open:      Button,

    branch: Label,

    changes: Changes,

    history: History,
}

impl Setup for MainView {
    fn setup(mut self: Weak<Self>) {
        Window::set_title("GitKukareker");

        self.repo_name.place().tl(20).size(400, 100);
        self.repo_name
            .custom_format(|path| path.file_name().unwrap().to_string_lossy().to_string());
        self.repo_name.on_changed(move |path| {
            self.repo_selected(&path);
        });

        self.open.set_text("Open");
        self.open
            .place()
            .same([X], self.repo_name)
            .anchor(Top, self.repo_name, 20)
            .size(200, 100);

        link_button!(self, open, on_open);

        self.branch.place().below(self.open, 20);

        self.changes.place().trb(0).w(500);

        self.history.place().t(200).b(0).lr(400);

        self.update();
    }
}

impl MainView {
    fn update(mut self: Weak<Self>) {
        self.repo_name.set_values(State::repos().collect());
        self.repo_selected(self.repo_name.value());
    }

    fn on_open(self: Weak<Self>) {
        let Some(result) = FileDialog::new().set_directory("/").pick_folder() else {
            return;
        };

        State::add_repo(result);
        self.update();
    }

    fn repo_selected(mut self: Weak<Self>, path: &Path) {
        let spin = Spinner::lock();

        let path = path.to_owned();

        Task::blocking(move || {
            let repo = Repo::open(path)?;

            Ok((
                RepoData {
                    changes: repo.changes()?,
                    history: repo.history()?,
                    branch:  repo.current_branch()?,
                },
                repo,
            ))
        })
        .callback(move |data: Result<(RepoData, Repo)>| {
            let Some((data, repo)) = data.alert_err() else {
                return;
            };

            self.repo = repo.into();

            self.changes.set_changes(data.changes);
            self.history.set_history(data.history);
            self.branch.set_text(data.branch);
            drop(spin);
        });
    }
}

struct RepoData {
    changes: Vec<Change>,
    history: Vec<CommitHistory>,
    branch:  String,
}
