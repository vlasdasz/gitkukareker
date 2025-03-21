use std::path::{Path, PathBuf};

use anyhow::Result;
use git::{Change, CommitHistory, Repo};
use rfd::FileDialog;
use rtools::Unwrap;
use test_engine::{
    Task, TaskSpinner, Window,
    refs::Weak,
    ui::{
        AlertErr,
        Anchor::{Top, X},
        Button, DropDown, HasText, Label, Setup, Spinner, ViewData, link_button, view,
    },
};

use crate::{
    model::State,
    ui::{changes::Changes, commit_view::CommitView, history::History},
};

#[view]
pub struct MainView {
    repo: Unwrap<Repo>,

    #[init]
    repo_name: DropDown<PathBuf>,
    open:      Button,

    branch: Label,

    fetch: Button,

    discard: Button,

    stage: Button,

    changes: Changes,

    history: History,

    commit: CommitView,
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

        self.fetch.set_text("Fetch");
        self.fetch.place().below(self.branch, 20);
        self.fetch.on_tap(move || {
            Task::spin(move || self.repo.fetch());
        });

        self.discard.set_text("Discard All");
        self.discard.place().below(self.fetch, 20);
        self.discard.on_tap(move || {
            Task::spin(move || self.repo.discard_all());
        });

        self.stage.set_text("Stage All");
        self.stage.place().tr(0).size(500, 100);
        self.stage.on_tap(move || {
            Task::spin(move || self.repo.stage_all());
        });

        self.changes.place().anchor(Top, self.stage, 20).br(0).w(500);

        self.history.place().t(200).b(0).lr(550);

        self.commit.place().bl(0).size(600, 280);
        self.commit.on_push_pressed(move |message| {
            Task::spin(move || self.repo.commit(message));
        });

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
