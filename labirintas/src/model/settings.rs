use serde::{Deserialize, Serialize};
use test_engine::{educe::Educe, reflected, reflected::Reflected};

use crate::ui::SETTINGS;

#[derive(Educe, Reflected, Serialize, Deserialize)]
#[educe(Default)]
pub struct Settings {
    #[educe(Default = true)]
    show_timer:   bool,
    gyro_control: bool,
}

impl Settings {
    pub fn show_timer() -> bool {
        SETTINGS.get().show_timer
    }
}
