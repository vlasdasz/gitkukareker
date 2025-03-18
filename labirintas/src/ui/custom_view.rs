use test_engine::{
    refs::Weak,
    ui::{view, Button, HasText, HasTitle, Labeled, NumberView, Setup, Size, ViewData, ViewTransition},
};

use crate::{
    model::LevelType,
    ui::{GameView, MainMenu},
};

#[view]
struct Buttons {
    #[init]
    width:  Labeled<NumberView<usize>>,
    height: Labeled<NumberView<usize>>,
    start:  Button,
}

#[view]
pub struct CustomView {
    #[init]
    buttons: Buttons,
    back:    Button,
}

impl Setup for CustomView {
    fn setup(mut self: Weak<Self>) {
        self.buttons.place().all_sides(100).all_ver();

        self.buttons.start.add_transition::<Buttons, GameView>();

        self.back.place().size(100, 50).t(200).l(50);
        self.back.add_transition::<Self, MainMenu>().set_text("Back");

        self.buttons.width.input.set_min(2);
        self.buttons.height.input.set_min(2);

        self.buttons.width.set_title("Width");
        self.buttons.height.set_title("Height");
        self.buttons.start.set_text("Start");
    }
}

impl ViewTransition<GameView> for Buttons {
    fn transition_to(self: Weak<Self>, game: &mut GameView) {
        game.level_type = LevelType::Custom;
        game.level_data.size = Size::new(self.width.input.value(), self.height.input.value());
    }
}
