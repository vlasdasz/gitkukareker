use std::{any::Any, ops::Deref};

use test_engine::{
    refs::{Own, Weak},
    ui::{view, CollectionData, CollectionView, Setup, View, ViewData},
};

use crate::{model::Change, ui::changes::changes_cell::ChangesCell};

#[view]
pub struct Changes {
    changes: Vec<Change>,

    #[init]
    table: CollectionView,
}

impl Changes {
    pub fn set_changes(&mut self, changes: Vec<Change>) {
        self.changes = changes;
        self.table.reload_data();
    }
}

impl CollectionData for Changes {
    fn number_of_cells(&self) -> usize {
        self.changes.len()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let cell = cell.downcast_mut::<ChangesCell>().unwrap();
        cell.set_change(&self.changes[index]);
    }

    fn make_cell(&self) -> Own<dyn View> {
        ChangesCell::new()
    }
}

impl Setup for Changes {
    fn setup(self: Weak<Self>) {
        self.table.set_data_source(self.deref());
        self.table.place().back();
    }
}
