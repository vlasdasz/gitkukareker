use test_engine::{
    refs::Weak,
    store::OnDisk,
    ui::{view, Button, FormView, HasText, Setup, ViewData, ViewSubviews},
};

use crate::{model::Settings, ui::MainMenu};

pub static SETTINGS: OnDisk<Settings> = OnDisk::new("settings");

#[view]
pub struct SettingsView {
    #[init]
    form: FormView<Settings>,
}

impl Setup for SettingsView {
    fn setup(mut self: Weak<Self>) {
        self.form.place().all_sides(200);
        self.form.set_data(&SETTINGS.get());

        self.add_view::<Button>()
            .add_transition::<Self, MainMenu>()
            .set_text("Back")
            .place()
            .tl(120)
            .size(100, 50);

        self.form.on_change.sub(move || {
            SETTINGS.set(self.form.get_data());
        });
    }
}
