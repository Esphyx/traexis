use super::super::queue::tetromino::Tetromino;
use strum::EnumCount;

pub enum Axis {
    X,
    Y,
    Z,
}

const N: usize = 4;
// pub const SHAPES: [[[[[[bool; N]; N]; N]; super::Turn::COUNT]; super::Face::COUNT];
// Tetromino::COUNT] = [[], [], [], [], [], [], [], []];

const fn orientations(shape: [[[bool; N]; N]; N]) {}

const fn rotate<const N: usize>(shape: [[[bool; N]; N]; N], axis: Axis) -> [[[bool; N]; N]; N] {
    let mut result = [[[false; N]; N]; N];

    let mut i = 0;
    while i < N {
        let mut j = 0;
        while j < N {
            let mut k = 0;
            while k < N {
                match axis {
                    Axis::X => result[i][j][N - k - 1] = shape[k][j][i],
                    Axis::Y => result[i][N - j - 1][k] = shape[k][j][i],
                    Axis::Z => result[N - i - 1][j][k] = shape[k][j][i],
                }

                k += 1;
            }
            j += 1;
        }
        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::piece::shapes::{rotate, Axis};

    #[test]
    pub fn rotator() {
        let shape = [
            [
                [false, false],
                [false, false]
            ],
            [
                [false, false],
                [false, false]
            ],
        ];

        println!("{:?}", shape);
        println!("X:");
        println!("{:?}", rotate(shape, Axis::X));
        println!("Y:");
        println!("{:?}", rotate(shape, Axis::Y));
        println!("Z:");
        println!("{:?}", rotate(shape, Axis::Z));

        assert_eq!(1, 1);
    }
}
