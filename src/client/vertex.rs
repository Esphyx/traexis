#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn from_uv(position: [f32; 3], uv: [f32; 2]) -> Self {
        Self {
            position,
            color: [0.; 3],
            uv,
        }
    }
}

impl From<[f32; 3]> for Vertex {
    fn from(value: [f32; 3]) -> Self {
        Self {
            position: value,
            color: [0.0; 3],
            uv: [0.; 2],
        }
    }
}

glium::implement_vertex!(Vertex, position, color, uv);
