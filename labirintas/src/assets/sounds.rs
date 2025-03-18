use test_engine::{audio::Sound, refs::Weak, DataManager};

pub struct Sounds;

impl Sounds {
    pub fn pek() -> Weak<Sound> {
        Sound::load(include_bytes!("sounds/pek.wav"), "labirintas::pek.wav")
    }
}
