use strum::EnumCount;

use crate::tetromino::Tetromino;

use super::{Axis, Turn};

pub const SIZE: usize = 4;
pub const SHAPES: [[[u64; Turn::COUNT]; Axis::COUNT]; Tetromino::COUNT] = [
    generate_orientations(
        0b0000_0000_0000_0000_0010_0010_0010_0010_0000_0000_0000_0000_0000_0000_0000_0000,
    ),
    generate_orientations(
        0b0000_0000_0000_0000_0000_0000_0110_0000_0000_0000_0110_0000_0000_0000_0000_0000,
    ),
    generate_orientations(
        0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    ),
    generate_orientations(
        0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    ),
    generate_orientations(
        0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    ),
    generate_orientations(
        0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    ),
    generate_orientations(
        0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    ),
    generate_orientations(
        0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000,
    ),
];

pub const fn generate_orientations(shape: u64) -> [[u64; Turn::COUNT]; Axis::COUNT] {
    let mut orientations = [[0; Turn::COUNT]; Axis::COUNT];

    let mut direction = 0;
    while direction < Axis::COUNT {
        let axis = match Axis::from_repr(direction) {
            Some(axis) => axis,
            None => unreachable!(),
        };

        let mut base_shape = axis.align(shape);

        let mut turn = 0;
        while turn < Turn::COUNT {
            orientations[direction][turn] = base_shape;

            base_shape = rotate_bitboard(base_shape, axis);
            turn += 1;
        }

        direction += 1;
    }

    orientations
}

// positive axis -> rotation anti-clockwise
pub const fn rotate_bitboard(shape: u64, axis: Axis) -> u64 {
    let mut result = 0;

    #[inline]
    const fn index(x: usize, y: usize, z: usize) -> usize {
        x + z * SIZE + y * SIZE * SIZE
    }

    let mut y = 0;
    while y < 4 {
        let mut z = 0;
        while z < 4 {
            let mut x = 0;
            while x < 4 {
                let is_set = shape >> index(x, y, z) & 1;
                let i = match axis {
                    Axis::PosX => index(x, z, SIZE - y - 1),
                    Axis::PosY => index(z, y, SIZE - x - 1),
                    Axis::PosZ => index(y, SIZE - x - 1, z),
                    Axis::NegX => index(x, SIZE - z - 1, y),
                    Axis::NegY => index(SIZE - z - 1, y, x),
                    Axis::NegZ => index(SIZE - y - 1, x, z),
                };
                result |= is_set << i;

                x += 1;
            }
            z += 1;
        }
        y += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{rotate_bitboard, Axis};

    #[test]
    pub fn rotate() {
        //                  :               |                   |                   |
        let bb = 0b0000000000000000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0011_0010;
        //              0000000000000010000000000000001100000000000000000000000000000000

        let rotated = rotate_bitboard(bb, Axis::PosX);
        println!("{:064b}", rotated);
    }
}
