#[derive(Default, Debug, Copy, Clone)]
pub enum LevelType {
    Classic,
    Custom,

    #[default]
    None,
}

impl LevelType {
    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }
}
