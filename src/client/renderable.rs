use traexis_core::{
    piece::{shapes, Piece},
    tetromino::Tetromino,
    State,
};

use super::{linear_algebra::add, vertex::Vertex};

pub trait Renderable {
    fn to_vertices(&self) -> Vec<Vertex>;
}

impl Renderable for State<{ crate::WIDTH }, { crate::HEIGHT }, { crate::DEPTH }> {
    fn to_vertices(&self) -> Vec<Vertex> {
        let [x_start, y_start, x_end, y_end] = self.current_piece.tetromino.get_sprite_bounds();
        let [x_start, y_start, x_end, y_end] = [x_start, 1. - y_start, x_end, 1. - y_end];

        let cube = [
            // face neg z
            Vertex::from_uv([1.0, 0.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 1.0, 0.0], [x_start, y_start]),
            Vertex::from_uv([0.0, 0.0, 0.0], [x_start, y_end]),
            Vertex::from_uv([1.0, 0.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 1.0, 0.0], [x_start, y_start]),
            Vertex::from_uv([1.0, 1.0, 0.0], [x_end, y_start]),
            // face neg y
            Vertex::from_uv([1.0, 0.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 0.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([0.0, 0.0, 0.0], [x_start, y_end]),
            Vertex::from_uv([1.0, 0.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 0.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([1.0, 0.0, 1.0], [x_end, y_start]),
            // face neg x
            Vertex::from_uv([0.0, 1.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 0.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([0.0, 0.0, 0.0], [x_start, y_end]),
            Vertex::from_uv([0.0, 1.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 0.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([0.0, 1.0, 1.0], [x_end, y_start]),
            // face pos z
            Vertex::from_uv([1.0, 0.0, 1.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 1.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([0.0, 0.0, 1.0], [x_start, y_end]),
            Vertex::from_uv([1.0, 0.0, 1.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 1.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([1.0, 1.0, 1.0], [x_end, y_start]),
            // face pos y
            Vertex::from_uv([1.0, 1.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 1.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([0.0, 1.0, 0.0], [x_start, y_end]),
            Vertex::from_uv([1.0, 1.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([0.0, 1.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([1.0, 1.0, 1.0], [x_end, y_start]),
            // face pos x
            Vertex::from_uv([1.0, 1.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([1.0, 0.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([1.0, 0.0, 0.0], [x_start, y_end]),
            Vertex::from_uv([1.0, 1.0, 0.0], [x_end, y_end]),
            Vertex::from_uv([1.0, 0.0, 1.0], [x_start, y_start]),
            Vertex::from_uv([1.0, 1.0, 1.0], [x_end, y_start]),
        ];

        let mut vertices = Vec::new();

        for y in 0..crate::HEIGHT {
            let bitboard = self.playfield[y];
            for x in 0..crate::WIDTH {
                for z in 0..crate::DEPTH {
                    if bitboard.get(x, z) {
                        cube.iter().for_each(|cube_vertex| {
                            vertices.push(Vertex {
                                position: add([x as f32, y as f32, z as f32], cube_vertex.position),
                                color: Tetromino::garbage_color(),
                                uv: cube_vertex.uv,
                            })
                        }); 
                    }
                }
            }
        }

        let current_shape = self.current_piece.get_shape();
        let [x, y, z] = self.current_piece.position;

        for u in 0..shapes::SIZE {
            for v in 0..shapes::SIZE {
                for w in 0..shapes::SIZE {
                    if Piece::contains_mino(current_shape, u, v, w) {
                        cube.iter().for_each(|cube_vertex| {
                            vertices.push(Vertex {
                                position: add(
                                    [u as f32, v as f32, w as f32],
                                    add(cube_vertex.position, [x as f32, y as f32, z as f32]),
                                ),
                                color: self.current_piece.tetromino.get_color(),
                                uv: cube_vertex.uv,
                            })
                        });
                    }
                }
            }
        }

        vertices
    }
}
