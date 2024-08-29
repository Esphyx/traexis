use super::linear_algebra::{add, cross, dot, negate, normalize, rodrigues, subtract};

#[derive(Debug)]
pub struct Camera {
    pub eye: [f32; 3],
    pub target: [f32; 3],
    pub up: [f32; 3],
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[allow(unused)]
impl Camera {
    #[inline]
    pub fn forward(&self) -> [f32; 3] {
        subtract(self.target, self.eye)
    }

    #[inline]
    pub fn back(&self) -> [f32; 3] {
        subtract(self.eye, self.target)
    }

    #[inline]
    pub fn right(&self) -> [f32; 3] {
        cross(self.up, self.forward())
    }

    #[inline]
    pub fn left(&self) -> [f32; 3] {
        negate(self.right())
    }

    #[inline]
    pub fn up(&self) -> [f32; 3] {
        self.up
    }

    #[inline]
    pub fn down(&self) -> [f32; 3] {
        negate(self.up)
    }

    pub fn mv(&mut self, change: [f32; 3]) {
        self.eye = add(self.eye, change);
        self.target = add(self.target, change);
    }
}

impl Camera {
    pub fn process_mouse(&mut self, delta_x: f32, delta_y: f32) {
        self.set_direction(rodrigues(
            rodrigues(subtract(self.target, self.eye), self.up, delta_x),
            self.right(),
            delta_y,
        ));
    }

    pub fn set_direction(&mut self, direction: [f32; 3]) {
        self.target = add(self.eye, direction);
    }

    pub fn get_direction(&self) -> [f32; 3] {
        subtract(self.target, self.eye)
    }

    pub fn perspective(&self, width: u32, height: u32) -> [[f32; 4]; 4] {
        let aspect_ratio = height as f32 / width as f32;
        let f = 1.0 / (self.fov / 2.0).tan();

        [
            [f * aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [
                0.0,
                0.0,
                (self.zfar + self.znear) / (self.zfar - self.znear),
                1.0,
            ],
            [
                0.0,
                0.0,
                -(2.0 * self.zfar * self.znear) / (self.zfar - self.znear),
                0.0,
            ],
        ]
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let forward = normalize(subtract(self.target, self.eye));
        let right = normalize(cross(self.up, forward));
        let true_up = cross(forward, right);

        [
            [right[0], true_up[0], forward[0], 0.0],
            [right[1], true_up[1], forward[1], 0.0],
            [right[2], true_up[2], forward[2], 0.0],
            [
                -dot(right, self.eye),
                -dot(true_up, self.eye),
                -dot(forward, self.eye),
                1.0,
            ],
        ]
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: [0.0, 0.0, 2.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fov: 90f32.to_radians(),
            znear: 0.1,
            zfar: 1024.0,
        }
    }
}
