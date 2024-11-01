use glium::{glutin::surface::WindowSurface, Display};

pub fn get_atlas_texture(display: &Display<WindowSurface>) -> glium::Texture2d {
    let atlas_image = image::load(
        std::io::Cursor::new(&include_bytes!("../../assets/atlas.png")),
        image::ImageFormat::Png,
    )
    .expect("Could not load image")
    .to_rgba8();

    let dimensions = atlas_image.dimensions();
    let atlas_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&atlas_image.into_raw(), dimensions);
    glium::texture::Texture2d::new(display, atlas_image).expect("Could not create the texture!")
}
