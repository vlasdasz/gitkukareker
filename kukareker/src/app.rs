use test_engine::{
    refs::Own,
    ui::{Setup, View},
    App,
};

use crate::ui::main_view::MainView;

pub struct KukarekerApp;

impl App for KukarekerApp {
    fn new() -> Self
    where Self: Sized {
        Self
    }

    fn setup(&self) {}

    fn make_root_view(&self) -> Own<dyn View> {
        MainView::new()
    }
}
