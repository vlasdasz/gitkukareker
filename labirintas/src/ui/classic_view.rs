use std::{any::Any, ops::Deref, time::Duration};

use test_engine::{
    refs::Weak,
    ui::{
        view, Button, CollectionData, CollectionView, HasText, Label, Setup, TextAlignment, UIManager,
        ViewData,
    },
};

use crate::{
    model::{Classic, LevelType},
    ui::{GameView, MainMenu},
};

#[view]
pub struct ClassicView {
    data: Classic,

    #[init]
    back:  Button,
    table: CollectionView,
}

impl Setup for ClassicView {
    fn setup(mut self: Weak<Self>) {
        self.data = Classic::get();
        self.back
            .add_transition::<Self, MainMenu>()
            .set_text("Back")
            .place()
            .size(100, 50)
            .t(200)
            .l(10);
        self.table.place().all_sides(200);
        self.table.set_data_source(self.deref());
    }
}

impl CollectionData for ClassicView {
    fn number_of_cells(&self) -> usize {
        self.data.levels.len()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let cell = cell.downcast_mut::<Label>().unwrap();
        cell.alignment = TextAlignment::Left;
        let level = &self.data.levels[index];
        cell.set_text(format!(
            "{}x{} - {}",
            level.size.width,
            level.size.height,
            format_duration(level.record)
        ));
    }

    fn cell_selected(&mut self, index: usize) {
        let level = &self.data.levels[index];
        let mut view = GameView::new();
        view.level_type = LevelType::Classic;
        view.level_data = *level;
        UIManager::set_view(view);
    }
}

pub fn format_duration(duration: Duration) -> String {
    if duration.as_secs() >= 31_536_000 {
        return "Have not finished".to_string();
    }

    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let millis = duration.subsec_millis();

    format!("{minutes:02}:{seconds:02}.{millis:02}")
}
