#[macro_use]
extern crate glium;

mod application;
mod layer;
mod objects;
mod utils;

use application::{Application, context::AppState};

fn main() {
    AppState::<Application>::run_loop();
}
