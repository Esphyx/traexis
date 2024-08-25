pub struct Camera {
    position: [f32; 3],
    target: [f32; 3],
    up: [f32; 3],
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Default::default(),
            target: Default::default(),
            up: Default::default(),
        }
    }
}
