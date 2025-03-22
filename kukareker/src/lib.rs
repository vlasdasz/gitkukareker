#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(const_trait_impl)]
#![feature(duration_constructors)]

use crate::app::KukarekerApp;

mod app;
mod model;
mod ui;

test_engine::register_app!(KukarekerApp);
