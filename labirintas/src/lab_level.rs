use std::{f32::consts::PI, time::Duration};

use test_engine::{
    generate::maze::{Cell, CellSide, Grid, Maker},
    gm::{Apply, Direction, LossyConvert, Shape},
    level::{
        level, CoefficientCombineRule, Control, LevelBase, LevelCreation, LevelManager, LevelSetup,
        LevelTemplates, Player, Sensor, Sprite, SpriteTemplates, Wall,
    },
    refs::{weak_from_ref, Weak},
    time::Instant,
    ui::{Alert, Point, Question, Setup, UIManager},
};

use crate::{
    assets::{Images, Sounds},
    model::{Classic, LevelData, LevelType},
    ui::{GameView, MainMenu},
};

#[level]
pub struct LabLevel {
    level:      LevelData,
    level_type: LevelType,

    cells: Vec<Weak<Wall>>,

    pub player: Weak<Player>,

    pub finished:    bool,
    pub started:     Instant,
    pub finish_time: Duration,
}

impl LabLevel {
    pub fn new(level: LevelData, level_type: LevelType) -> Self {
        Self {
            __level_base: LevelBase::default(),
            level,
            level_type,
            cells: vec![],
            started: Instant::now(),
            finished: false,
            finish_time: Duration::default(),
            player: Weak::default(),
        }
    }
}

impl LevelSetup for LabLevel {
    fn setup(&mut self) {
        self.background = Images::wood_background();
        self.set_gravity((0, 0));
        self.setup_player();
        weak_from_ref(self).display_grid(Maker::generate(self.level.size));
    }

    fn update(&mut self) {
        let pos = self.player.position();
        *LevelManager::camera_pos() = pos;
    }

    fn needs_physics(&self) -> bool {
        true
    }
}

impl LabLevel {
    fn setup_player(&mut self) {
        self.player = self.make_sprite(Shape::Circle(6.0), (0, 0));
        self.player.tag = 777;

        self.player.weapon.bullet_image = Images::metal_ball();
        self.player.weapon.bullet_speed = 10.0;
        self.player.unit.body.set_friction(0);

        self.player.set_image(Images::metal_ball()).unit.body.unlock_rotation();
        self.player.unit.set_restitution(0.2, CoefficientCombineRule::Min);

        self.player.unit.enable_collision_detection();
        self.player.on_collision.sub(move || {
            Sounds::pek().play();
        });

        let mut this = weak_from_ref(self);
        self.__level_base.on_tap.val(move |pos| this.player.weapon.weak().shoot_at(pos));

        this.player.unit.body.jump_force = 10.0;

        [
            (' ', Direction::Up),
            ('w', Direction::Up),
            ('s', Direction::Down),
            ('d', Direction::Right),
            ('a', Direction::Left),
        ]
        .apply(|(key, direction)| {
            UIManager::keymap().add(this, key, move || {
                this.player.unit.body.move_by_direction(direction);
            });
        });
    }

    fn display_grid(mut self: Weak<Self>, grid: Grid) {
        self.cells.iter_mut().for_each(|a| a.remove());
        self.cells.clear();

        for (x, row) in grid.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                self.add_cell(*cell, x, y, x == grid.len() - 1 && y == row.len() - 1);
            }
        }
    }

    fn finish_classic(self: Weak<Self>) {
        let last_level = Classic::is_last_level(self.level);

        if !last_level {
            let new_record = Classic::update_record(self.level, self.finish_time);

            let message = if new_record { "New record!" } else { "You won!" };

            Alert::show_callback(message, move || {
                UIManager::set_view(MainMenu::new());
            });

            return;
        }

        Classic::advance_level(self.finish_time);

        Question::ask(format!(
            "You won level {}x{}!\nContinue to next level?",
            self.level.size.width, self.level.size.height
        ))
        .options("Main Menu", "Continue")
        .callback(move |next| {
            if next {
                let mut lab = GameView::new();
                lab.level_type = LevelType::Classic;
                lab.level_data = Classic::get_level();
                UIManager::set_view(lab);
            } else {
                UIManager::set_view(MainMenu::new());
            }
        });
    }

    fn add_cell(mut self: Weak<Self>, cell: Cell, x: usize, y: usize, last: bool) {
        // if !cell.visited {
        //     let mut wall = self.add_sprite::<Wall>((SIZE, SIZE), origin(x, y));
        //     wall.set_color(Color::BLACK);
        //     self.cells.push(wall);
        // }

        cell.all_sides(|side| {
            let trigger = last && matches!(side, CellSide::Right);

            if !trigger {
                let mut wall = self.make_sprite::<Wall>(
                    Shape::Rect((BIG + SMALL, SMALL).into()),
                    position_for_side(side, x, y),
                );
                wall.set_rotation(rotation_for_side(side));
                wall.set_image(Images::wood_wall());
                self.cells.push(wall);
                return;
            }

            let mut pos = position_for_side(side, x, y);
            pos.x += 10.0;
            let mut sensor = self.make_sprite::<Sensor>(Shape::rect(5, 5), pos);
            sensor.tag = 10;

            sensor.on_collision.sub(move || {
                if self.finished {
                    return;
                }

                self.finished = true;
                self.finish_time = self.started.elapsed();

                match self.level_type {
                    LevelType::Classic => {
                        self.finish_classic();
                    }
                    LevelType::Custom => {
                        Alert::show_callback("You won!", move || {
                            UIManager::set_view(MainMenu::new());
                        });
                    }
                    LevelType::None => {
                        unreachable!("Level type: None")
                    }
                }
            });
        });
    }
}

const BIG: f32 = 20.0;
const SIDE_SHIFT: f32 = BIG / 2.0;
const SMALL: f32 = 4.4;

fn position_for_side(side: CellSide, x: usize, y: usize) -> Point {
    let (x, y) = (x.lossy_convert() * BIG, y.lossy_convert() * BIG);

    match side {
        CellSide::Up => (x, y + SIDE_SHIFT).into(),
        CellSide::Down => (x, y - SIDE_SHIFT).into(),
        CellSide::Left => (x - SIDE_SHIFT, y).into(),
        CellSide::Right => (x + SIDE_SHIFT, y).into(),
    }
}

fn rotation_for_side(side: CellSide) -> f32 {
    match side {
        CellSide::Up | CellSide::Down => 0.0,
        CellSide::Left | CellSide::Right => PI * 0.5,
    }
}
