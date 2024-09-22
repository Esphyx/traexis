mod app;
mod client;
mod config;

use app::*;

const WIDTH: usize = 4;
const HEIGHT: usize = 20;
const DEPTH: usize = 4;

fn main() {
    let config = config::Config::read("config.json");
    println!("{:?}", config);

    App::new(config.window_title, config.fullscreen);
}
