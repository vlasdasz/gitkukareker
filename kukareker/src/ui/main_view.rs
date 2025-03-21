use std::path::PathBuf;

use git::Repo;
use rfd::FileDialog;
use rtools::Unwrap;
use test_engine::{
    Window,
    refs::Weak,
    ui::{
        Anchor::{Top, X},
        Button, DropDown, HasText, Label, Setup, ViewData, link_button, view,
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
            self.repo_selected(&path).unwrap();
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
        self.repo_selected(self.repo_name.value()).unwrap();
    }

    fn on_open(self: Weak<Self>) {
        let Some(result) = FileDialog::new().set_directory("/").pick_folder() else {
            return;
        };

        State::add_repo(result);
        self.update();
    }

    fn repo_selected(mut self: Weak<Self>, path: &PathBuf) -> anyhow::Result<()> {
        self.repo = Repo::open(path)?.into();

        let changes = self.repo.changes()?;
        self.changes.set_changes(changes);

        let history = self.repo.history()?;
        self.history.set_history(history);

        let branch = self.repo.current_branch()?;
        self.branch.set_text(branch);

        Ok(())
    }
}
