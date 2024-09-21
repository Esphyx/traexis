#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl From<[f32; 3]> for Vertex {
    fn from(value: [f32; 3]) -> Self {
        Self {
            position: value,
            color: [0.5; 3],
        }
    }
}

glium::implement_vertex!(Vertex, position, color);
