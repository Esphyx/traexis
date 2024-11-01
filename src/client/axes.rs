use super::vertex::Vertex;

#[allow(unused)]
pub fn get_axes() -> Vec<Vertex> {
    const SIZE: f32 = 4.;
    vec![
        Vertex {
            position: [-SIZE, 0., 0.],
            color: [1., 0., 0.],
            uv: [0.; 2],
        },
        Vertex {
            position: [SIZE, 0., 0.],
            color: [1., 0., 0.],
            uv: [0.; 2],
        },
        Vertex {
            position: [0., -SIZE, 0.],
            color: [0., 1., 0.],
            uv: [0.; 2],
        },
        Vertex {
            position: [0., SIZE, 0.],
            color: [0., 1., 0.],
            uv: [0.; 2],
        },
        Vertex {
            position: [0., 0., -SIZE],
            color: [0., 0., 1.],
            uv: [0.; 2],
        },
        Vertex {
            position: [0., 0., SIZE],
            color: [0., 0., 1.],
            uv: [0.; 2],
        },
    ]
}
