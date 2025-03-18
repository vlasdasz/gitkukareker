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

use crate::model::State;

#[view]
pub struct Main {
    #[init]
    repo_name: DropDown,
    open:      Button,
}

impl Setup for Main {
    fn setup(mut self: Weak<Self>) {
        Window::set_title("GitKukareker");

        self.repo_name.place().tl(20).size(400, 100);

        self.open.set_text("Open");
        self.open
            .place()
            .same([X], self.repo_name)
            .anchor(Top, self.repo_name, 20)
            .size(200, 100);

        link_button!(self, open, on_open);

        self.update();
    }
}

impl Main {
    fn update(mut self: Weak<Self>) {
        self.repo_name
            .set_values(State::repos().map(|repo| repo.file_name().unwrap().to_string_lossy().to_string()));
    }

    fn on_open(self: Weak<Self>) {
        let Some(result) = FileDialog::new().set_directory("/").pick_folder() else {
            return;
        };

        State::add_repo(result);
        self.update();
    }
}
