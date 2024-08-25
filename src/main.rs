mod config;
mod graphics;

fn main() {
    let config = config::Config::read("config.json");
    println!("{:?}", config);

    graphics::app::TreaxisApp::new(config.window_title, config.fullscreen);
}
