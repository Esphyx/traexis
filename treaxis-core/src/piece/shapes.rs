use super::{super::queue::tetromino::Tetromino, Axis, Turn};
use strum::EnumCount;

const N: usize = 4;
pub const SHAPES: [[[[[[bool; N]; N]; N]; super::Turn::COUNT]; Axis::COUNT]; Tetromino::COUNT] = [
    compute_orientations([
        // I
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ]),
    compute_orientations([
        // O
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ]),
    compute_orientations([
        // T
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ]),
    compute_orientations([
        // L
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ]),
    compute_orientations([
        // S
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ]),
    compute_orientations([
        // B
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, false, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ]),
    compute_orientations([
        // D
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, true, false],
            [false, false, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ]),
    compute_orientations([
        // F
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, true, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ]),
];

const fn compute_orientations(
    shape: [[[bool; N]; N]; N],
) -> [[[[[bool; N]; N]; N]; super::Turn::COUNT]; Axis::COUNT] {
    let mut orientations = [[[[[false; N]; N]; N]; super::Turn::COUNT]; Axis::COUNT];

    let mut i = 0;
    while i < Axis::COUNT {
        let base_shape = match i {
            0 => rotate(shape, Axis::PosZ, 1), // to right
            1 => shape,                        // to top
            2 => rotate(shape, Axis::PosX, 1), // to front
            3 => rotate(shape, Axis::NegZ, 1), // to left
            4 => rotate(shape, Axis::PosX, 2), // to bottom
            5 => rotate(shape, Axis::NegX, 1), // to back
            _ => unreachable!(),
        };

        let mut turn = 0;
        while turn < Turn::COUNT {
            // cross of base_shape axis and up
            let rotation_axis = match i {
                0 => Axis::PosX, // right
                1 => Axis::PosY, // top
                2 => Axis::PosZ, // front
                3 => Axis::NegX, // left
                4 => Axis::NegY, // bottom
                5 => Axis::NegZ, // back
                _ => unreachable!(),
            };

            orientations[i][turn] = rotate(base_shape, rotation_axis, turn);

            turn += 1;
        }
        i += 1;
    }

    orientations
}

const fn rotate<const N: usize>(
    shape: [[[bool; N]; N]; N],
    axis: Axis,
    n: usize,
) -> [[[bool; N]; N]; N] {
    let mut result = shape;

    let mut iteration = 0;
    while iteration < n {
        let mut i = 0;
        while i < N {
            let mut j = 0;
            while j < N {
                let mut k = 0;
                while k < N {
                    match axis {
                        Axis::PosX => result[i][k][N - j - 1] = shape[i][j][k],
                        Axis::PosY => result[N - k - 1][j][i] = shape[i][j][k],
                        Axis::PosZ => result[j][N - i - 1][k] = shape[i][j][k],
                        Axis::NegX => result[i][N - k - 1][j] = shape[i][j][k],
                        Axis::NegY => result[k][j][N - i - 1] = shape[i][j][k],
                        Axis::NegZ => result[N - j - 1][i][k] = shape[i][j][k],
                    }

                    k += 1;
                }
                j += 1;
            }
            i += 1;
        }
        iteration += 1;
    }

    result
}
