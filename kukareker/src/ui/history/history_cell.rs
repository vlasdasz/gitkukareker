use test_engine::{
    refs::Weak,
    ui::{view, HasText, Label, Setup, ViewData},
};

use crate::ui::history::commit_history::CommitHistory;

#[view]
pub struct HistoryCell {
    #[init]
    author:  Label,
    email:   Label,
    hash:    Label,
    message: Label,
}

impl HistoryCell {
    pub fn set_commit(&mut self, commit: &CommitHistory) {
        self.author.set_text(&commit.author);
        self.email.set_text(&commit.email);
        self.hash.set_text(&commit.hash);
        self.message.set_text(&commit.message);
    }
}

impl Setup for HistoryCell {
    fn setup(self: Weak<Self>) {
        self.place().all_hor();
    }
}
