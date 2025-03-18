use test_engine::{
    gm::Apply,
    level::LevelManager,
    refs::Weak,
    ui::{
        view,
        Anchor::{Bot, Top},
        Button, Color, Consent, HasText, Setup, ViewData,
    },
    Window,
};

use crate::{
    model::Classic,
    ui::{
        classic_view::ClassicView,
        styles::{BACKGROUND_COLOR, BIG_BUTTON_STYLE},
        CustomView, SettingsView,
    },
};

#[view]
pub struct MainMenu {
    #[init]
    custom: Button,

    classic:  Button,
    settings: Button,

    reset: Button,
}

impl Setup for MainMenu {
    fn setup(mut self: Weak<Self>) {
        Window::set_clear_color(BACKGROUND_COLOR);
        LevelManager::stop_level();

        self.reset.place().t(200).l(100).size(100, 50);
        self.reset
            .set_text("Reset")
            .set_corner_radius(10)
            .set_color((51, 89, 218))
            .set_text_color(Color::WHITE);
        self.reset.on_tap(|| {
            Consent::ask("Are you sure you want to reset your progress?", |yes| {
                if yes {
                    Classic::reset();
                }
            });
        });

        [self.custom, self.classic, self.settings].apply(|mut view| {
            view.add_style(BIG_BUTTON_STYLE);
        });

        self.custom
            .add_transition::<Self, CustomView>()
            .set_text("Custom")
            .place()
            .size(520, 140)
            .center();

        self.classic.add_transition::<Self, ClassicView>().set_text("Classic");

        self.classic
            .place()
            .same_size(self.custom)
            .center_x()
            .anchor(Bot, self.custom, 60);

        self.settings.add_transition::<Self, SettingsView>().set_text("Settings");
        self.settings
            .place()
            .same_size(self.custom)
            .center_x()
            .anchor(Top, self.custom, 60);
    }
}
