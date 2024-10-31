mod app;
mod client;
mod config;

use app::*;

const WIDTH: usize = 8;
const HEIGHT: usize = 20;
const DEPTH: usize = 8;

fn main() {
    let config = config::Config::read("config.json");

    App::new(config.window_title, config.fullscreen);
}
