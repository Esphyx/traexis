use std::collections::HashSet;

use glium::winit::keyboard::KeyCode;

use super::linear_algebra::{add, cross, dot, negate, normalize, rodrigues, scale, subtract};

#[derive(Debug)]
pub struct Camera {
    eye: [f32; 3],
    target: [f32; 3],
    up: [f32; 3],
    fov: f32,
    znear: f32,
    zfar: f32,
    pub velocity: [f32; 3],
    pub acceleration: [f32; 3],
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

// PROCESS INPUT
impl Camera {
    pub fn process_keys(&mut self, keys: &HashSet<KeyCode>) {
        keys.iter().for_each(|&key| {
            self.acceleration = add(
                self.acceleration,
                match key {
                    KeyCode::KeyW => self.forward(),
                    KeyCode::KeyA => self.left(),
                    KeyCode::KeyS => self.back(),
                    KeyCode::KeyD => self.right(),
                    KeyCode::ShiftLeft => self.down(),
                    KeyCode::Space => self.up(),
                    _ => [0.; 3],
                },
            );
        });
    }

    pub fn update(&mut self, delta_time: f32) {
        let scale_factor = delta_time * 4.;
        self.velocity = add(self.velocity, scale(self.acceleration, scale_factor));
        self.mv(scale(self.velocity, scale_factor));

        let damping = 0.95;
        self.velocity = scale(self.velocity, damping);
        self.acceleration = [0.; 3];
    }

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
}

// MATRICES
impl Camera {
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
        let eye = [-4., 8., -4.];
        let target = add(eye, [0., 0., 1.]);
        Self {
            eye,
            target,
            up: [0.0, 1.0, 0.0],
            fov: 90f32.to_radians(),
            znear: 0.01,
            zfar: 1024.0,
            velocity: [0.; 3],
            acceleration: [0.; 3],
        }
    }
}
