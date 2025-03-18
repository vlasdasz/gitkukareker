#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]
#![feature(duration_constructors)]

mod assets;
mod lab_level;
mod model;
mod ui;

fn main() {
    test_engine::launch_app!();
}
