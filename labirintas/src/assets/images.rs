#![allow(dead_code)]

pub struct Images;
use test_engine::{refs::Weak, ui::Image, DataManager};

impl Images {
    pub fn _arrow() -> Weak<Image> {
        Image::load(include_bytes!("images/arrow.png"), "labirintas::arrow")
    }

    pub fn ball() -> Weak<Image> {
        Image::load(include_bytes!("images/ball.png"), "labirintas::ball")
    }

    pub fn sky() -> Weak<Image> {
        Image::load(include_bytes!("images/sky.png"), "labirintas::sky")
    }

    pub fn glass_tile() -> Weak<Image> {
        Image::load(include_bytes!("images/Glass_Tile.png"), "labirintas::glass_tile")
    }

    pub fn metal_ball() -> Weak<Image> {
        Image::load(include_bytes!("images/Metal_Ball.png"), "labirintas::metal_ball")
    }

    pub fn metal_ball_2() -> Weak<Image> {
        Image::load(
            include_bytes!("images/Metal_Ball_2.png"),
            "labirintas::metal_ball_2",
        )
    }

    pub fn ball_connected() -> Weak<Image> {
        Image::load(
            include_bytes!("images/Ball_Connected.png"),
            "labirintas::ball_connected",
        )
    }

    pub fn ball_dust() -> Weak<Image> {
        Image::load(include_bytes!("images/Ball_Dust.png"), "labirintas::ball_dust")
    }

    pub fn ball_scratched() -> Weak<Image> {
        Image::load(
            include_bytes!("images/Ball_Scratched.png"),
            "labirintas::ball_scratched",
        )
    }

    pub fn wood_background() -> Weak<Image> {
        Image::load(
            include_bytes!("images/Wood_Background.png"),
            "labirintas::wood_background",
        )
    }

    pub fn wood_tile() -> Weak<Image> {
        Image::load(include_bytes!("images/Wood_Tile.png"), "labirintas::wood_tile")
    }

    pub fn wood_wall() -> Weak<Image> {
        Image::load(include_bytes!("images/Wood_Wall.png"), "labirintas::wood_wall")
    }
}
