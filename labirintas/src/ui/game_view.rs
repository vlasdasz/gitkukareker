use std::{any::Any, ops::Deref};

use test_engine::{
    gm::{Apply, Direction},
    level::{Control, LevelManager, LevelTemplates, Sprite, SpriteTemplates},
    refs::{weak_from_ref, Own, Weak},
    ui::{
        view, Anchor::Top, Button, CollectionData, CollectionView, HasText, Image, ImageView, Label, Setup,
        Size, StickView, UIManager, View, ViewCallbacks, ViewData,
    },
};

use crate::{
    assets::Images,
    lab_level::LabLevel,
    model::{LevelData, LevelType, Settings},
    ui::{classic_view::format_duration, MainMenu},
};

#[view]
pub struct GameView {
    pub level_type: LevelType,
    pub level_data: LevelData,

    level: Weak<LabLevel>,

    balls: Vec<Weak<Image>>,

    #[init]
    level_title: Label,
    time_passed: Label,

    stick: StickView,
    back:  Button,

    balls_view: CollectionView,
}

impl GameView {
    fn setup_level(&mut self) {
        assert!(!self.level_type.is_none(), "Set level type");

        self.level = LevelManager::set_level(LabLevel::new(self.level_data, self.level_type));

        self.setup_keymap();
    }

    fn setup_keymap(&mut self) {
        let mut this = weak_from_ref(self);

        UIManager::keymap().add(this, '=', || {
            *LevelManager::scale() *= 2.0;
        });

        UIManager::keymap().add(this, '-', || {
            *LevelManager::scale() /= 2.0;
        });

        [
            (' ', Direction::Up),
            ('w', Direction::Up),
            ('s', Direction::Down),
            ('d', Direction::Right),
            ('a', Direction::Left),
        ]
        .apply(|(key, direction)| {
            UIManager::keymap().add(this, key, move || {
                this.level.player.unit.body.move_by_direction(direction);
            });
        });
    }
}

impl Setup for GameView {
    fn setup(mut self: Weak<Self>) {
        self.balls = vec![
            Images::ball_connected(),
            Images::ball_dust(),
            Images::ball_scratched(),
            Images::metal_ball(),
            Images::metal_ball_2(),
            Images::ball(),
        ];

        self.balls_view.place().t(120).r(0).size(100, 800);
        self.balls_view.set_data_source(self.deref());

        let name = self.level_data.name();
        self.level_title.set_text(name).place().size(200, 50).center_x().t(100);

        self.time_passed
            .place()
            .size(200, 50)
            .center_x()
            .anchor(Top, self.level_title, 10);

        self.time_passed.set_hidden(!Settings::show_timer());

        self.back.place().tl(200).size(100, 50);
        self.back.set_text("Exit").set_text_size(40);
        self.back.on_tap(|| {
            UIManager::set_view(MainMenu::new());
        });

        self.stick.place().size(200, 200).center_x().b(200);

        self.stick.on_change.val(move |direction| {
            if LevelManager::no_level() {
                return;
            }
            let mut level = LevelManager::level_weak();
            level.set_gravity(direction.invert_y() * 10.0);
            LevelManager::downcast_level::<LabLevel>()
                .player
                .unit
                .rigid_body_mut()
                .wake_up(true);
        });

        self.setup_level();
    }
}

impl ViewCallbacks for GameView {
    fn update(&mut self) {
        if self.level.finished {
            self.time_passed.set_text(format_duration(self.level.finish_time));
        } else {
            self.time_passed.set_text(format_duration(self.level.started.elapsed()));
        }
    }
}

impl CollectionData for GameView {
    fn number_of_cells(&self) -> usize {
        self.balls.len()
    }

    fn setup_cell_for_index(&self, cell: &mut dyn Any, index: usize) {
        let image_view = cell.downcast_mut::<ImageView>().unwrap();
        image_view.set_image(self.balls[index]);
    }

    fn size_for_index(&self, _index: usize) -> Size {
        (100, 100).into()
    }

    fn make_cell(&self) -> Own<dyn View> {
        ImageView::new()
    }

    fn cell_selected(&mut self, index: usize) {
        LevelManager::downcast_level::<LabLevel>().player.set_image(self.balls[index]);
    }
}
