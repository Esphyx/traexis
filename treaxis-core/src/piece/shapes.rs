use super::super::queue::tetromino::Tetromino;
use strum::EnumCount;

#[derive(Clone, Copy)]
pub enum Axis {
    PosX,
    PosY,
    PosZ,
    NegX,
    NegY,
    NegZ,
}

const N: usize = 4;
pub const SHAPES: [[[[[[bool; N]; N]; N]; super::Turn::COUNT]; super::Face::COUNT];
    Tetromino::COUNT] = [
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
) -> [[[[[bool; N]; N]; N]; super::Turn::COUNT]; super::Face::COUNT] {
    let mut orientations = [[[[[false; N]; N]; N]; super::Turn::COUNT]; super::Face::COUNT];

    let mut face = 0;
    while face < super::Face::COUNT {
        let base_shape = match face {
            0 => rotate(shape, Axis::PosX),
            1 => rotate(shape, Axis::NegX),
            2 => rotate(shape, Axis::PosZ),
            3 => rotate(shape, Axis::NegZ),
            4 => shape,
            5 => rotate(rotate(shape, Axis::PosX), Axis::PosX),
            _ => shape,
        };

        let mut turn = 0;
        while turn < super::Turn::COUNT {
            let axis = match face {
                0 => Axis::NegZ,
                1 => Axis::PosZ,
                2 => Axis::NegX,
                3 => Axis::PosX,
                4 => Axis::NegY,
                5 => Axis::PosY,
                _ => Axis::NegY,
            };

            let rotated_shape = match turn {
                0 => base_shape,
                1 => rotate(base_shape, axis),
                2 => rotate(rotate(base_shape, axis), axis),
                3 => rotate(rotate(rotate(base_shape, axis), axis), axis),
                _ => base_shape,
            };

            orientations[face][turn] = rotated_shape;

            turn += 1;
        }
        face += 1;
    }

    orientations
}

const fn rotate<const N: usize>(shape: [[[bool; N]; N]; N], axis: Axis) -> [[[bool; N]; N]; N] {
    let mut result = [[[false; N]; N]; N];

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

    result
}
