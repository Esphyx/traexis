use treaxis_core::State;

use super::{linear_algebra::add, vertex::Vertex};

pub trait Renderable {
    fn to_vertices(&self) -> Vec<Vertex>;
}

impl Renderable for State<{ crate::WIDTH }, { crate::HEIGHT }, { crate::DEPTH }> {
    fn to_vertices(&self) -> Vec<Vertex> {
        let cube = [
            // face neg z
            Vertex::from([1.0, 0.0, 0.0]),
            Vertex::from([0.0, 1.0, 0.0]),
            Vertex::from([0.0, 0.0, 0.0]),
            Vertex::from([1.0, 0.0, 0.0]),
            Vertex::from([0.0, 1.0, 0.0]),
            Vertex::from([1.0, 1.0, 0.0]),
            // face neg y
            Vertex::from([1.0, 0.0, 0.0]),
            Vertex::from([0.0, 0.0, 0.0]),
            Vertex::from([0.0, 0.0, 1.0]),
            Vertex::from([1.0, 0.0, 0.0]),
            Vertex::from([0.0, 0.0, 1.0]),
            Vertex::from([1.0, 0.0, 1.0]),
            // face neg x
            Vertex::from([0.0, 0.0, 0.0]),
            Vertex::from([0.0, 1.0, 0.0]),
            Vertex::from([0.0, 0.0, 1.0]),
            Vertex::from([0.0, 1.0, 0.0]),
            Vertex::from([0.0, 0.0, 1.0]),
            Vertex::from([0.0, 1.0, 1.0]),
            // face pos z
            Vertex::from([1.0, 0.0, 1.0]),
            Vertex::from([0.0, 1.0, 1.0]),
            Vertex::from([0.0, 0.0, 1.0]),
            Vertex::from([1.0, 0.0, 1.0]),
            Vertex::from([0.0, 1.0, 1.0]),
            Vertex::from([1.0, 1.0, 1.0]),
            // face pos y
            Vertex::from([1.0, 1.0, 0.0]),
            Vertex::from([0.0, 1.0, 0.0]),
            Vertex::from([0.0, 1.0, 1.0]),
            Vertex::from([1.0, 1.0, 0.0]),
            Vertex::from([0.0, 1.0, 1.0]),
            Vertex::from([1.0, 1.0, 1.0]),
            // face pos x
            Vertex::from([1.0, 0.0, 0.0]),
            Vertex::from([1.0, 1.0, 0.0]),
            Vertex::from([1.0, 0.0, 1.0]),
            Vertex::from([1.0, 1.0, 0.0]),
            Vertex::from([1.0, 0.0, 1.0]),
            Vertex::from([1.0, 1.0, 1.0]),
        ];

        let mut vertices = Vec::new();

        for y in 0..crate::HEIGHT {
            let bitboard = self.playfield[y];
            for x in 0..crate::WIDTH {
                for z in 0..crate::DEPTH {
                    if bitboard.get(x, z) {
                        cube.iter().for_each(|cube_vertex| {
                            vertices.push(Vertex::from(add(
                                [x as f32, y as f32, z as f32],
                                cube_vertex.position,
                            )))
                        });
                    }
                }
            }
        }

        vertices
    }
}
