use crate::{DEPTH, HEIGHT, WIDTH};

use super::vertex::Vertex;

pub fn get_grid_lines() -> Vec<Vertex> {
    let mut grid_lines = Vec::new();

    for i in 0..(WIDTH + 1) {
        grid_lines.push(Vertex::from([i as f32, 0., DEPTH as f32]));
        grid_lines.push(Vertex::from([i as f32, HEIGHT as f32, DEPTH as f32]));

        
        grid_lines.push(Vertex::from([i as f32, 0., 0.]));
        grid_lines.push(Vertex::from([i as f32, 0., DEPTH as f32]));
    }

    for i in 0..(HEIGHT + 1) {
        grid_lines.push(Vertex::from([0., i as f32, 0.]));
        grid_lines.push(Vertex::from([0., i as f32, DEPTH as f32]));

        grid_lines.push(Vertex::from([0., i as f32, DEPTH as f32]));
        grid_lines.push(Vertex::from([WIDTH as f32, i as f32, DEPTH as f32]));
    }

    for i in 0..(DEPTH + 1) {
        grid_lines.push(Vertex::from([0., 0., i as f32]));
        grid_lines.push(Vertex::from([0., HEIGHT as f32, i as f32]));


        grid_lines.push(Vertex::from([0., 0., i as f32]));
        grid_lines.push(Vertex::from([WIDTH as f32, 0., i as f32]));
    }

    grid_lines.iter_mut().for_each(|v| (*v).color = [1.; 3]);
    grid_lines
}
