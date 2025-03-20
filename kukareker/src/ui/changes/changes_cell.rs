use git::Change;
use test_engine::{
    refs::Weak,
    ui::{Anchor::Left, HasText, Label, Setup, ViewData, view},
};

#[view]
pub struct ChangesCell {
    #[init]
    status: Label,
    file:   Label,
}

impl ChangesCell {
    pub fn set_change(&mut self, change: &Change) {
        self.status.set_text(change.status.to_string());
        self.file.set_text(&change.file.to_string());
    }
}

impl Setup for ChangesCell {
    fn setup(self: Weak<Self>) {
        self.status.place().tlb(0).w(40);

        self.file.place().anchor(Left, self.status, 10).trb(0);
    }
}
