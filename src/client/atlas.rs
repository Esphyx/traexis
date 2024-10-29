use glium::{glutin::surface::WindowSurface, Display};
use image::ImageBuffer;

const SPRITE_SIZE: usize = 12;
const ATLAS_WIDTH: usize = SPRITE_SIZE * 4;

fn create_atlas(paths: &[&'static str]) {

}

fn load_assets(display: &Display<WindowSurface>) -> glium::Texture2d {
    let image = image::load(
        std::io::Cursor::new(&include_bytes!("../../assets/red.png")),
        image::ImageFormat::Png,
    )
    .expect("Could not load image")
    .to_rgba8();

    let dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dimensions);
    glium::texture::Texture2d::new(display, image).expect("Could not create the texture!")
}
