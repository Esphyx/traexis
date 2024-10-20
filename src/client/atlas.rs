use glium::texture::RawImage2d;

fn load_assets<'a>() -> RawImage2d<'a, u8> {
    let red = image::load(
        std::io::Cursor::new(&include_bytes!("../../assets/red.png")),
        image::ImageFormat::Png,
    )
    .expect("Could not load image")
    .to_rgba8();

    let dimensions = red.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(&red.into_raw(), dimensions)
}
