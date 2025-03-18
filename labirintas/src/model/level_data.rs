use std::time::Duration;

use serde::{Deserialize, Serialize};
use test_engine::ui::Size;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct LevelData {
    pub size:   Size<usize>,
    pub record: Duration,
}

impl LevelData {
    pub fn name(&self) -> String {
        format!("{} X {}", self.size.width, self.size.height)
    }
}
