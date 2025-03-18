use test_engine::ui::{Button, HasText, Style, U8Color, ViewData, ViewSubviews};

pub const BACKGROUND_COLOR: U8Color = U8Color::const_rgb(32, 34, 37);

pub const BIG_BUTTON_STYLE: Style = Style::new(|btn| {
    btn.apply_if::<Button>(|btn| {
        btn.set_corner_radius(28);
        btn.set_text_size(80);
    });
});
