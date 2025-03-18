use test_engine::{
    refs::Own,
    ui::{Button, HasText, Setup, Style, U8Color, View, ViewData, ViewSubviews},
    App,
};

use crate::ui::MainMenu;

pub const BUTTON_COLOR: U8Color = U8Color::const_rgb(51, 89, 218);

pub const BUTTON_STYLE: Style = Style::new(|btn| {
    btn.set_corner_radius(12);
    btn.set_color(BUTTON_COLOR);

    btn.apply_if::<Button>(|btn| {
        btn.set_text_color(test_engine::ui::Color::WHITE);
    });
});

pub struct LabirintasApp;

impl App for LabirintasApp {
    fn new() -> Self
    where Self: Sized {
        Self
    }

    fn setup(&self) {
        BUTTON_STYLE.apply_to_all::<Button>();
    }

    fn make_root_view(&self) -> Own<dyn View> {
        MainMenu::new()
    }
}
