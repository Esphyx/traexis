use treaxis_core::State;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
}

glium::implement_vertex!(Vertex, position);

pub trait Renderable {
    fn to_vertices(&self) -> Vec<Vertex>;
}

impl Renderable for State<{ crate::WIDTH }, { crate::HEIGHT }, { crate::DEPTH }> {
    fn to_vertices(&self) -> Vec<Vertex> {
        vec![
            Vertex {
                position: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 1.0],
            },
        ]
    }
}
