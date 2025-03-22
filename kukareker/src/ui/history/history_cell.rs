use git::CommitHistory;
use test_engine::{
    refs::Weak,
    ui::{HasText, Label, Setup, ViewData, view},
};

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
