use std::{any::Any, ops::Deref};

use test_engine::{
    refs::{Own, Weak},
    ui::{view, CollectionData, CollectionView, Setup, View, ViewData},
};

use crate::ui::history::{commit_history::CommitHistory, history_cell::HistoryCell};

#[view]
pub struct History {
    history: Vec<CommitHistory>,

    #[init]
    table: CollectionView,
}

impl History {
    pub fn set_history(&mut self, history: Vec<CommitHistory>) {
        self.history = history;
        self.table.reload_data();
    }
}

impl CollectionData for History {
    fn number_of_cells(&self) -> usize {
        self.history.len()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let cell = cell.downcast_mut::<HistoryCell>().unwrap();
        cell.set_commit(&self.history[index]);
    }

    fn make_cell(&self) -> Own<dyn View> {
        HistoryCell::new()
    }
}

impl Setup for History {
    fn setup(self: Weak<Self>) {
        self.table.set_data_source(self.deref());
        self.table.place().back();
    }
}
