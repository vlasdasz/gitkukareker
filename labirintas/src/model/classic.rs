use std::{cmp::min, time::Duration};

use serde::{Deserialize, Serialize};
use test_engine::{store::OnDisk, ui::Size};

use crate::model::LevelData;

pub static CLASSIC: OnDisk<Classic> = OnDisk::new("classic");

#[derive(Debug, Serialize, Deserialize)]
pub struct Classic {
    pub levels: Vec<LevelData>,
}

impl Classic {
    pub fn get() -> Self {
        CLASSIC.get()
    }

    pub fn get_level() -> LevelData {
        *Self::get().levels.last().unwrap()
    }

    pub fn is_last_level(level: LevelData) -> bool {
        Self::get_level().size == level.size
    }

    pub fn update_record(level: LevelData, finish_time: Duration) -> bool {
        let mut data = Self::get();
        let level = data.levels.iter_mut().find(|l| l.size == level.size).expect("Level not found");
        let record = level.record > finish_time;
        level.record = min(level.record, finish_time);
        CLASSIC.set(data);
        record
    }

    pub fn advance_level(finish_time: Duration) {
        let mut data = Self::get();
        let last_level = data.levels.last_mut().unwrap();
        last_level.record = min(last_level.record, finish_time);
        let new_size = Size::new(last_level.size.width + 1, last_level.size.height + 1);
        let _ = last_level;
        data.levels.push(LevelData {
            size:   new_size,
            record: Duration::from_days(365),
        });
        CLASSIC.set(data);
    }

    pub fn reset() {
        CLASSIC.reset();
    }
}

impl Default for Classic {
    fn default() -> Self {
        Self {
            levels: vec![LevelData {
                size:   Size::new(1, 1),
                record: Duration::from_days(365),
            }],
        }
    }
}
