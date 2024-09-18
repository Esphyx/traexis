use super::linear_algebra::{add, cross, dot, negate, normalize, rodrigues, subtract};

#[derive(Debug)]
pub struct Camera {
    eye: [f32; 3],
    target: [f32; 3],
    up: [f32; 3],
    fov: f32,
    znear: f32,
    zfar: f32,
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
        cross(self.up, self.back())
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
        self.target = add(
            self.eye,
            rodrigues(
                rodrigues(subtract(self.target, self.eye), self.up, delta_x),
                self.right(),
                delta_y,
            ),
        );
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
        let eye = [5.0, 5.0, 5.0];
        let target = [eye[0], eye[1], eye[2] + 1.0];
        Self {
            eye,
            target,
            up: [0.0, 1.0, 0.0],
            fov: 90f32.to_radians(),
            znear: 0.01,
            zfar: 1024.0,
        }
    }
}
