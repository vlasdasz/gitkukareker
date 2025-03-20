use test_engine::{
    refs::Weak,
    ui::{view, Anchor::Left, HasText, Label, Setup, ViewData},
};

use crate::model::Change;

#[view]
pub struct ChangesCell {
    #[init]
    status: Label,
    file:   Label,
}

impl ChangesCell {
    pub fn set_change(&mut self, change: &Change) {
        self.status.set_text(change.status);
        self.file.set_text(&change.file);
    }
}

impl Setup for ChangesCell {
    fn setup(self: Weak<Self>) {
        self.status.place().tlb(0).w(40);

        self.file.place().anchor(Left, self.status, 10).trb(0);
    }
}
