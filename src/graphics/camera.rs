pub struct Camera {
    position: [f32; 3],
    target: [f32; 3],
    up: [f32; 3],
    fov: f32,
}

impl Camera {
    pub fn perspective(&self, width: u32, height: u32) -> [[f32; 4]; 4] {
        let aspect_ratio = height as f32 / width as f32;
        let focal_length = 1.0 / (self.fov / 2.0).tan();

        const ZNEAR: f32 = 0.1;
        const ZFAR: f32 = 1024.0;

        [
            [focal_length * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, focal_length, 0.0, 0.0],
            [0.0, 0.0, (ZFAR + ZNEAR) / (ZFAR - ZNEAR), 1.0],
            [0.0, 0.0, -(2.0 * ZFAR * ZNEAR) / (ZFAR - ZNEAR), 0.0],
        ]
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let z_axis = -
    }
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
