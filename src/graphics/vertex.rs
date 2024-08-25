#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
}

glium::implement_vertex!(Vertex, position);
